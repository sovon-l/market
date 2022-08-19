#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WssMessage {
    // r#type: String,
    // channel: String,
    market: String,
    data: Option<Vec<Trade>>,
}

// {"id": 3522050233, "price": 38380.0, "size": 0.0002, "side": "sell", "liquidation": false, "time": "2022-03-15T09:05:05.234634+00:00"}
#[derive(Serialize, Deserialize, Debug)]
struct Trade {
    id: u64,
    price: rust_decimal::Decimal,
    size: rust_decimal::Decimal,
    side: String,
    time: String,
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
                    r#"{{"op":"subscribe","channel":"trades","market":"{}"}}"#,
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
    crate::util::websocket::Message<()>,
) -> Option<tokio_tungstenite::tungstenite::Message> {
    move |time_recv, _, msg| {
        let msg = match msg {
            crate::util::websocket::Message::Control(_) => return None,
            crate::util::websocket::Message::WssMessage(msg) => msg,
        };
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
            let mut trades: Vec<proper_ma_structs::structs::market::trades::Trade> = data
                .into_iter()
                .map(|t| proper_ma_structs::structs::market::trades::Trade {
                    price: t.price,
                    size: {
                        let mut rt = t.size;
                        rt.set_sign_positive(t.side != "buy");
                        rt
                    },
                    timestamp: chrono::DateTime::parse_from_rfc3339(&t.time)
                        .unwrap()
                        .timestamp_nanos() as u64,
                })
                .collect();

            for (n, i) in trades.iter_mut().enumerate() {
                i.timestamp += n as u64;
            }

            sender
                .send(crate::message::Message::TradesMsg(
                    proper_ma_structs::structs::market::trades::Trades {
                        symbol,
                        market_timestamp: time_recv
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as u64,
                        trades,
                    },
                ))
                .unwrap();
        }
        None
    }
}
