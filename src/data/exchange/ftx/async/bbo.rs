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
    pub insts: Vec<proper_ma_structs::structs::market::instrument::Instrument>,
}

impl crate::util::websocket::WssState for State {
    fn init_messages(&self) -> Vec<tokio_tungstenite::tungstenite::Message> {
        self.insts
            .iter()
            .map(|s| {
                tokio_tungstenite::tungstenite::Message::text(format!(
                    r#"{{"op":"subscribe","channel":"ticker","market":"{}"}}"#,
                    crate::data::exchange::ftx::serde::se_inst(&s)
                ))
            })
            .collect()
    }
}

pub fn wss(
    mut sender: impl messenger::traits::ChannelSender<crate::message::Message>,
) -> impl FnMut(
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
            let symbol = if let Some(s) = crate::data::exchange::ftx::serde::de_inst(&market) {
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
                .send(crate::message::Message::QuotesMsg(
                    proper_ma_structs::structs::market::quotes::Quotes {
                        symbol,
                        market_timestamp: time_recv
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as u64,
                        timestamp: Some((time * 1_000_000_000.0) as u64),
                        is_snapshot: true,
                        is_l1: true,
                        depths: vec![
                            proper_ma_structs::structs::market::quotes::Depth {
                                price: bid,
                                size: bidSize,
                            },
                            proper_ma_structs::structs::market::quotes::Depth {
                                price: ask,
                                size: askSize,
                            },
                        ],
                    },
                ))
                .unwrap();
        }
        None
    }
}
