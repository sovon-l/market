#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WssMessage {
    // r#type: String,
    // channel: String,
    market: String,
    data: Option<WssData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WssData {
    time: f64,
    bid: rust_decimal::Decimal,
    bidSize: rust_decimal::Decimal,
    ask: rust_decimal::Decimal,
    askSize: rust_decimal::Decimal,
}

pub struct State {
    pub insts: Vec<crate::structs::symbol::Symbol>,
}

impl crate::util::websocket::WssState for State {
    fn init_messages(&self) -> Vec<tokio_tungstenite::tungstenite::Message> {
        self.insts
            .iter()
            .map(|s| {
                tokio_tungstenite::tungstenite::Message::text(format!(
                    r#"{{"op":"subscribe","channel":"ticker","market":"{}"}}"#,
                    crate::data::exchange::ftx::serde::se_symbol(s)
                ))
            })
            .collect()
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
        let WssMessage { market, data } = msg;
        if let Some(data) = data {
            let WssData {
                time,
                bid,
                bidSize,
                ask,
                askSize,
            } = data;
            let symbol = if let Some(s) = crate::data::exchange::ftx::serde::de_symbol(&market) {
                s
            } else {
                log::error!(
                    "{}",
                    serde_json::json!({
                        "err": {
                            "msg": "error parsing symbol",
                            "rt": market,
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
                        timestamp: Some((time * 1_000_000_000.0) as u64),
                        bid_price: bid,
                        bid_size: bidSize,
                        ask_price: ask,
                        ask_size: askSize,
                    },
                ))
                .unwrap();
        }
        None
    }
}
