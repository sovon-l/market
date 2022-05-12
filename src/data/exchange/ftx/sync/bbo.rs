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
                log::error!("error parsing symbol: {}", market);
                return;
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
    }
}
