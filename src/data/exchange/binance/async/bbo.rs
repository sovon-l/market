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

pub struct State;

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
    tokio_tungstenite::tungstenite::Message,
) -> Option<tokio_tungstenite::tungstenite::Message> {
    move |time_recv, _, msg| {
        let msg: WssMessage = match msg {
            tokio_tungstenite::tungstenite::Message::Text(mut s) => {
                log::debug!(
                    "{}",
                    serde_json::json!({
                        "recv": s.to_string(),
                    })
                );
                match simd_json::serde::from_str(&mut s) {
                    Ok(m) => m,
                    Err(e) => {
                        log::error!(
                            "{}",
                            serde_json::json!({
                                "recv": s.to_string(),
                                "err": {
                                    "msg": "cannot parse",
                                    "rt": e.to_string(),
                                }
                            })
                        );
                        return None;
                    }
                }
            }
            tokio_tungstenite::tungstenite::Message::Ping(ts) => {
                return Some(tokio_tungstenite::tungstenite::Message::Pong(ts));
            }
            x => {
                log::error!(
                    "{}",
                    serde_json::json!({
                        "err": {
                            "msg": "unexpected_msg_type",
                            "rt": format!("{:?}", x),
                        }
                    })
                );
                return None;
            }
        };
        let WssMessage { data, .. } = msg;
        let WssData { s, b, B, a, A, .. } = data;
        let symbol = if let Some(s) = crate::data::exchange::binance::serde::de_inst(&s) {
            s
        } else {
            log::error!(
                "{}",
                serde_json::json!({
                    "err": {
                        "msg": "error parsing symbol",
                        "rt": s,
                    }
                })
            );
            return None;
        };
        sender
            .send(crate::message::Message::BboMsg(
                crate::structs::market_price::MarketPrice {
                    symbol,
                    market_timestamp: time_recv
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64,
                    timestamp: None,
                    bid_price: b,
                    bid_size: B,
                    ask_price: a,
                    ask_size: A,
                },
            ))
            .unwrap();
        None
    }
}
