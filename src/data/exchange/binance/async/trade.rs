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
    t: u64,
    b: u64,
    M: bool,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "T")]
    trade_ts: u64,
    #[serde(rename = "a")]
    id: u64,
    #[serde(rename = "p")]
    price: rust_decimal::Decimal,
    #[serde(rename = "q")]
    quantity: rust_decimal::Decimal,
    #[serde(rename = "m")]
    buyer_is_maker: bool,
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
    mut sender: impl messenger::traits::ChannelSender<crate::message::Message>,
) -> impl FnMut(
    std::time::SystemTime,
    &mut State,
    crate::util::websocket::Message<()>,
) -> Option<tokio_tungstenite::tungstenite::Message> {
    move |time_recv, state, msg| {
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
        let WssMessage { data, .. } = msg;
        let WssData {
            trade_ts,
            symbol,
            price,
            mut quantity,
            buyer_is_maker,
            ..
        } = data;
        let symbol = if let Some(s) = crate::data::exchange::binance::serde::de_inst(&symbol) {
            s
        } else {
            log::error!(
                "{}",
                serde_json::json!({
                    "err": {
                        "msg": "error parsing symbol",
                        "rt": symbol,
                    }
                })
            );
            return None;
        };
        quantity.set_sign_positive(!buyer_is_maker);
        let new_n = if let Some((last_ts, n)) = state.last_ts.as_ref() {
            if *last_ts == trade_ts {
                n + 1
            } else {
                0
            }
        } else {
            0
        };
        state.last_ts = Some((trade_ts, new_n));
        sender
            .send(crate::message::Message::TradesMsg(
                proper_ma_structs::structs::market::trades::Trades {
                    symbol,
                    market_timestamp: time_recv
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64,
                    trades: vec![proper_ma_structs::structs::market::trades::Trade {
                        price: price,
                        size: quantity,
                        timestamp: trade_ts * 1_000_000 + new_n,
                    }],
                },
            ))
            .unwrap();
        None
    }
}
