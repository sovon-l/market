pub trait WssState {
    fn init_messages(&self) -> Vec<tokio_tungstenite::tungstenite::Message>;
}

pub fn work<T: WssState>(
    address: &str,
    action: impl Fn(
        std::time::SystemTime,
        &mut T,
        &mut tokio_tungstenite::tungstenite::protocol::WebSocket<
            tokio_tungstenite::tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
        >,
        tokio_tungstenite::tungstenite::Message,
    ),
    mut state: T,
) {
    loop {
        log::debug!(
            "{}",
            serde_json::json!({
                "connecting": address,
            })
            .to_string()
        );
        let mut wss = match tokio_tungstenite::tungstenite::client::connect(address) {
            Ok((wss, resp)) => {
                log::info!(
                    "{}",
                    serde_json::json!({
                        "msg": "successfully connected",
                        "resp": format!("{:?}", resp),
                    })
                    .to_string()
                );
                wss
            }
            Err(e) => {
                log::error!(
                    "{}",
                    serde_json::json!({
                        "msg": "fail to connect",
                        "followup": "reconnecting in 1s",
                        "error": e.to_string(),
                    })
                    .to_string()
                );
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        };

        for msg in state.init_messages().into_iter() {
            wss.write_message(msg).unwrap();
        }

        while let Ok(msg) = wss.read_message() {
            let time = std::time::SystemTime::now();
            action(time, &mut state, &mut wss, msg);
            let time_used = std::time::SystemTime::now().duration_since(time).unwrap();
            if time_used > std::time::Duration::from_secs(1) {
                log::warn!(
                    "{}",
                    serde_json::json!({
                        "msg": "message action slow",
                        "time_used_us": time_used.as_micros(),
                    })
                    .to_string()
                );
            }
        }
    }
}

// TODO: should use annotation similar as above but async fn
pub async fn awork<T: WssState>(
    address: String,
    mut action: impl FnMut(
        std::time::SystemTime,
        &mut T,
        tokio_tungstenite::tungstenite::Message,
    ) -> Option<tokio_tungstenite::tungstenite::Message>,
    mut state: T,
) {
    use futures_util::sink::SinkExt;
    use futures_util::stream::StreamExt;
    loop {
        log::debug!(
            "{}",
            serde_json::json!({
                "connecting": address,
            })
            .to_string()
        );
        let mut wss = match tokio_tungstenite::connect_async(&address).await {
            Ok((wss, resp)) => {
                log::info!(
                    "{}",
                    serde_json::json!({
                        "msg": "successfully connected",
                        "resp": format!("{:?}", resp),
                    })
                    .to_string()
                );
                wss
            }
            Err(e) => {
                log::error!(
                    "{}",
                    serde_json::json!({
                        "msg": "fail to connect",
                        "followup": "reconnecting in 1s",
                        "error": e.to_string(),
                    })
                    .to_string()
                );
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        };

        for msg in state.init_messages().into_iter() {
            wss.send(msg).await.unwrap();
        }

        while let Some(msg) = wss.next().await {
            if let Ok(msg) = msg {
                let time = std::time::SystemTime::now();
                if let Some(msg) = action(time, &mut state, msg) {
                    wss.send(msg).await.unwrap();
                };
                let time_used = std::time::SystemTime::now().duration_since(time).unwrap();
                if time_used > std::time::Duration::from_secs(1) {
                    log::warn!(
                        "{}",
                        serde_json::json!({
                            "msg": "message action slow",
                            "time_used_us": time_used.as_micros(),
                        })
                        .to_string()
                    );
                }
            }
        }
    }
}
