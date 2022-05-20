#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WssMessage {
    stream: String,
    data: WssData,
}

#[derive(Serialize, Deserialize, Debug)]
struct WssData {
    e: String,
    E: u64,
    s: String,
    t: u64,
    p: rust_decimal::Decimal,
    q: rust_decimal::Decimal,
    b: u64,
    a: u64,
    T: u64,
    m: bool,
    M: bool,
}

#[derive(Default)]
pub struct State {
    last_ts: Option<(u64, u64)>,
}

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
    move |time_recv, state, msg| {
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
        let WssData {
            T, s, p, mut q, m, ..
        } = data;
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
        q.set_sign_positive(!m);
        let new_n = if let Some((last_ts, n)) = state.last_ts.as_ref() {
            if *last_ts == T {
                n + 1
            } else {
                0
            }
        } else {
            0
        };
        state.last_ts = Some((T, new_n));
        sender
            .send(crate::message::Message::TradesMsg(
                crate::structs::trades::Trades {
                    symbol,
                    market_timestamp: time_recv
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64,
                    trades: vec![crate::structs::trades::Trade {
                        price: p,
                        size: q,
                        timestamp: T * 1_000_000 + new_n,
                    }],
                },
            ))
            .unwrap();
        None
    }
}
