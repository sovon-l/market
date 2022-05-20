lazy_static::lazy_static! {
    pub static ref MARKET_PUBLISHER_INPUT_ADDRESS: String = std::env::var("MARKET_PUBLISHER_INPUT_ADDRESS").unwrap_or("0.0.0.0:7800".to_string());
    pub static ref MARKET_PUBLISHER_PUBLISH_ADDRESS: String = std::env::var("MARKET_PUBLISHER_PUBLISH_ADDRESS").unwrap_or("tcp://*:8000".to_string());
    pub static ref MARKET_PUBLISHER_TOKIO_THREAD: usize = std::env::var("MARKET_PUBLISHER_TOKIO_THREAD").unwrap_or("1".to_string()).parse::<usize>().unwrap();
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    simple_logger::SimpleLogger::new()
        .env()
        .with_utc_timestamps()
        .init()
        .unwrap();

    log::info!(
        "{}",
        serde_json::json!({"msg":"start async publisher"}).to_string()
    );

    let m = std::sync::Arc::new(std::sync::Mutex::new(zmq::Context::new()));

    let (sender, receiver) = crossbeam_channel::unbounded();
    let m_clone = m.clone();
    let _ = std::thread::spawn(move || {
        let ctx = m_clone.lock().unwrap();
        let socket = ctx.socket(zmq::PUB).unwrap();
        socket.bind(&MARKET_PUBLISHER_PUBLISH_ADDRESS).unwrap();

        messenger::publisher::publisher_loop(receiver, socket);
    });

    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(*MARKET_PUBLISHER_TOKIO_THREAD)
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            log::error!(
                "{}",
                serde_json::json!({
                    "error": e.to_string(),
                })
                .to_string()
            );
            panic!()
        }
    };
    runtime.block_on(amain(sender));
}

async fn amain(sender: crossbeam_channel::Sender<market::message::Message>) {
    use std::str::FromStr;
    let addr = std::net::SocketAddr::from_str(&MARKET_PUBLISHER_INPUT_ADDRESS).unwrap();

    let state = std::sync::Arc::new(tokio::sync::Mutex::new(
        market::data::controller::State::default(),
    ));

    let make_svc = hyper::service::make_service_fn(move |_conn| {
        let sender = sender.clone();
        let state = state.clone();
        async {
            Ok::<_, std::convert::Infallible>(hyper::service::service_fn(
                move |req: hyper::Request<hyper::Body>| {
                    let sender = sender.clone();
                    let state = state.clone();
                    async move {
                        let mut resp = hyper::Response::default();

                        let mut correct = false;
                        if let Ok(bytes) = hyper::body::to_bytes(req.into_body()).await {
                            let rt = serde_json::from_slice(&bytes);
                            if let Ok(rt) = rt {
                                log::info!(
                                    "{}",
                                    serde_json::json!({
                                        "http_recv": rt,
                                    })
                                    .to_string()
                                );
                                let rt: Vec<&str> = rt;
                                let insts = rt
                                    .iter()
                                    .map(|s| market::structs::instrument::Instrument::from_str(s))
                                    .filter(|v| v.is_ok())
                                    .map(|v| v.unwrap())
                                    .collect();
                                market::data::controller::work(sender, state, insts).await;
                                correct = true
                            }
                        }
                        if !correct {
                            *resp.status_mut() = hyper::StatusCode::BAD_REQUEST;
                        }
                        std::result::Result::<hyper::Response<hyper::Body>, std::convert::Infallible>::Ok(resp)
                    }
                },
            ))
        }
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    });

    if let Err(e) = graceful.await {
        log::error!("{}", e);
    }
}
