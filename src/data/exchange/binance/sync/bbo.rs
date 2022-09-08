#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WssMessage {
    stream: String,
    data: WssData,
}

#[derive(Serialize, Deserialize, Debug)]
struct WssData {
    u: u64,
    s: String,
    b: rust_decimal::Decimal,
    B: rust_decimal::Decimal,
    a: rust_decimal::Decimal,
    A: rust_decimal::Decimal,
}

pub struct State {}

impl crate::util::websocket::WssState for State {
    fn init_messages(&self) -> Vec<tokio_tungstenite::tungstenite::Message> {
        Vec::new()
    }
}

pub fn wss(
    sender: crossbeam_channel::Sender<crate::message::Message>,
) -> impl Fn(
    std::time::SystemTime,
    &mut State,
    &mut tokio_tungstenite::tungstenite::protocol::WebSocket<
        tokio_tungstenite::tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
    >,
    tokio_tungstenite::tungstenite::Message,
) {
    move |time_recv, _, wss, msg| {
        let msg: WssMessage = match msg {
            tokio_tungstenite::tungstenite::Message::Text(mut s) => {
                log::debug!("recv msg: {}", s);
                match simd_json::serde::from_str(&mut s) {
                    Ok(m) => m,
                    Err(e) => {
                        log::error!(
                            r#"{{"recv":"{}","err":{{"msg:"cannot parse","rt":"{}"}}}}"#,
                            s,
                            e
                        );
                        return;
                    }
                }
            }
            tokio_tungstenite::tungstenite::Message::Ping(ts) => {
                log::debug!("recv ping {:?}", ts);
                wss.write_message(tokio_tungstenite::tungstenite::Message::Pong(ts))
                    .unwrap();
                return;
            }
            x => {
                log::error!(r#"{{"err":{{"msg:"unexpected msg type","rt":"{:?}"}}}}"#, x);
                return;
            }
        };
        let WssMessage { data, .. } = msg;
        let WssData { s, b, B, a, A, .. } = data;
        let symbol = if let Some(s) = crate::data::exchange::binance::serde::de_inst(&s) {
            s
        } else {
            log::error!(r#"{{"err":{{"msg:"error parsing symbol","rt":"{}"}}}}"#, s);
            return;
        };
        sender
            .send(crate::message::Message::QuotesMsg(
                proper_ma_structs::structs::market::quotes::Quotes {
                    symbol,
                    market_timestamp: time_recv
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64,
                    timestamp: None,
                    is_snapshot: true,
                    is_l1: true,
                    depths: vec![
                        proper_ma_structs::structs::market::quotes::Depth { price: b, size: -B },
                        proper_ma_structs::structs::market::quotes::Depth { price: a, size: A },
                    ],
                },
            ))
            .unwrap();
    }
}
