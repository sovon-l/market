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
    pub insts: Vec<crate::structs::instrument::Instrument>,
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
                log::debug!(r#"{{"recv":{}}}"#, s);
                simd_json::serde::from_str(&mut s).unwrap()
            }
            tokio_tungstenite::tungstenite::Message::Ping(ts) => {
                wss.write_message(tokio_tungstenite::tungstenite::Message::Pong(ts))
                    .unwrap();
                return;
            }
            x => {
                println!("unexpected wss msg type: {:?}", x);
                return;
            }
        };
        let WssMessage { market, data } = msg;
        if let Some(data) = data {
            let symbol = if let Some(s) = crate::data::exchange::ftx::serde::de_inst(&market) {
                s
            } else {
                log::error!("error parsing symbol: {}", market);
                return;
            };
            let mut trades: Vec<crate::structs::trades::Trade> = data
                .into_iter()
                .map(|t| crate::structs::trades::Trade {
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
                    crate::structs::trades::Trades {
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
    }
}
