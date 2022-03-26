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
    &mut tokio_tungstenite::tungstenite::protocol::WebSocket<
        tokio_tungstenite::tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
    >,
    tokio_tungstenite::tungstenite::Message,
) {
    move |time_recv, state, wss, msg| {
        let msg: WssMessage = match msg {
            tokio_tungstenite::tungstenite::Message::Text(mut s) => {
                log::debug!(r#"{{"recv": "{}"}}"#, s);
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
        let WssData {
            T, s, p, mut q, m, ..
        } = data;
        let symbol = if let Some(s) = crate::data::exchange::binance::serde::de_symbol(&s) {
            s
        } else {
            log::error!(r#"{{"err":{{"msg:"error parsing symbol","rt":"{}"}}}}"#, s);
            return;
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
    }
}
