#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WssMessage {
    stream: String,
    data: WssData,
}

#[derive(Serialize, Deserialize, Debug)]
struct WssData {
    E: u64, // Event time
    s: String, // Symbol
    U: u64, // First update ID in event
    u: u64, // Final update ID in event
    // pu: Option<u64>, // Final update ID in previous event (only for binance derivatives)
    b: Vec<[rust_decimal::Decimal; 2]>, // Bids to be updated
    a: Vec<[rust_decimal::Decimal; 2]>, // Asks to be updated
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

struct UpdateSeqNum {
    first: u64,
    last: u64,
}
type MbpUpdate = crate::data::orderbook::MbpUpdate<UpdateSeqNum>;
struct SnapshotSeqNum {
    seq_num: u64,
}
type MbpFullbook = crate::data::orderbook::MbpFullbook<SnapshotSeqNum>;

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
        // bid < midprice, +ve quantity
        // ask > midprice, -ve quantity
        None
    }
}