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

    let sender = std::sync::Arc::new(sender);

    let app = axum::Router::new().route(
        "/",
        axum::routing::post({
            let state = std::sync::Arc::clone(&state);
            let sender = std::sync::Arc::clone(&sender);
            move |body| {
                monitor_instances(
                    body,
                    std::sync::Arc::clone(&state),
                    std::sync::Arc::clone(&sender),
                )
            }
        }),
    );

    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        log::error!(
            "{}",
            serde_json::json!({
                "error": e.to_string(),
            })
            .to_string()
        );
    };
}

async fn monitor_instances(
    axum::Json(payload): axum::Json<Vec<String>>,
    state: std::sync::Arc<tokio::sync::Mutex<market::data::controller::State>>,
    mut sender: std::sync::Arc<crossbeam_channel::Sender<market::message::Message>>,
) {
    use std::str::FromStr;
    log::info!("{}", serde_json::json!({"recv":payload}).to_string());
    let insts: Vec<market::structs::instrument::Instrument> = payload
        .iter()
        .map(|s| market::structs::instrument::Instrument::from_str(s))
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect();
    let sender = std::sync::Arc::make_mut(&mut sender);
    market::data::controller::work(sender.clone(), state, insts).await;
}
