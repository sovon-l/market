#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WssMessage {
    stream: String,
    data: WssData,
}

#[derive(Serialize, Deserialize, Debug)]
struct WssData {
    E: u64,    // Event time
    s: String, // Symbol
    U: u64,    // First update ID in event
    u: u64,    // Final update ID in event
    // pu: Option<u64>, // Final update ID in previous event (only for binance derivatives)
    b: Vec<[rust_decimal::Decimal; 2]>, // Bids to be updated
    a: Vec<[rust_decimal::Decimal; 2]>, // Asks to be updated
}

#[derive(Default)]
pub struct State {
    mbpcaches: std::collections::HashMap<
        proper_ma_structs::structs::market::instrument::Instrument,
        MbpCache,
    >,
}

impl crate::util::websocket::WssState for State {
    fn init_messages(&self) -> Vec<tokio_tungstenite::tungstenite::Message> {
        Vec::new()
    }
}

// seq num facilities

#[derive(Clone, Copy)]
pub struct UpdateSeqNum {
    first: u64,
    last: u64,
}
type MbpUpdate = crate::data::orderbook::MbpUpdate<UpdateSeqNum>;
#[derive(Clone, Copy)]
pub struct FullbookSeqNum {
    seq_num: u64,
}
impl crate::data::orderbook::VerificationStatus for FullbookSeqNum {
    type VerificationUpdate = UpdateSeqNum;
    fn verify(
        orderbook: &[crate::util::orderbook::depth::MbpDepth],
        status: &mut Self,
        update: &mut Self::VerificationUpdate,
    ) -> Option<bool> {
        if status.seq_num < update.first + 1 {
            return Some(false);
        }
        if status.seq_num < update.last {
            return None;
        }
        status.seq_num = update.last;
        Some(true)
    }
}
type MbpFullbook = crate::data::orderbook::MbpFullbook<FullbookSeqNum>;

// cacher

pub enum MbpCache {
    BuiltBook(MbpFullbook),
    BuildingBook(Vec<MbpUpdate>),
}
impl crate::data::mbp_handler::MbpCache<UpdateSeqNum, FullbookSeqNum> for MbpCache {
    fn is_book_built(&self) -> bool {
        match self {
            MbpCache::BuiltBook(_) => true,
            MbpCache::BuildingBook(_) => false,
        }
    }
    fn new_response(book: MbpFullbook) -> Self {
        MbpCache::BuiltBook(book)
    }
    fn new_update(update: MbpUpdate) -> Self {
        MbpCache::BuildingBook(vec![update])
    }
    fn process_response(
        &self,
        book: MbpFullbook,
    ) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        match self {
            MbpCache::BuiltBook(b) => {
                if book.verification_status.seq_num > b.verification_status.seq_num {
                    log::debug!("refresh book");
                    Ok(Some(MbpCache::BuiltBook(book)))
                } else {
                    log::warn!("unusual old refresh/response received, discarding");
                    Ok(None)
                }
            }
            MbpCache::BuildingBook(updates) => {
                let book_seq_num = book.verification_status.seq_num;
                let update_first_prev_seq_num = &updates[0].verification_update.first;
                let update_last_seq_num = &updates[updates.len() - 1].verification_update.last;

                if book_seq_num < *update_first_prev_seq_num {
                    log::debug!("seq num too small, discarding");
                    return Ok(None);
                }
                if *update_last_seq_num <= book_seq_num {
                    log::debug!("updates are too old, use book directly");
                    return Ok(Some(Self::BuiltBook(book)));
                }
                let mut mbp_book = Self::BuiltBook(book);
                let mut found = false;
                for update in updates {
                    if let Self::BuiltBook(mbp_book) = &mbp_book {
                        let update_prev_seq_num = update.verification_update.first;
                        let update_seq_num = update.verification_update.last;
                        let book_seq_num = mbp_book.verification_status.seq_num;

                        if update_prev_seq_num <= book_seq_num && book_seq_num < update_seq_num {
                            found = true;
                        }
                    }
                    if found {
                        log::info!("building book processing cached updates");
                        mbp_book.process_update(update.clone())?;
                    }
                }
                if !found {
                    return Err("cache.seq_num < resp.seq_num but not found".into());
                }
                Ok(Some(mbp_book))
            }
        }
    }
    fn process_update(&mut self, update: MbpUpdate) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            MbpCache::BuildingBook(u) => {
                if u.len() > 1000 {
                    Err("accumulated updates without refresh".into())
                } else {
                    u.push(update);
                    Ok(())
                }
            }
            MbpCache::BuiltBook(b) => {
                let update_prev_seq_num = update.verification_update.first;
                let update_seq_num = update.verification_update.last;
                let book_seq_num = b.verification_status.seq_num;

                if update_seq_num <= book_seq_num {
                    log::debug!(
                        "our {} their prev {} their new {} - discard",
                        book_seq_num,
                        update_prev_seq_num,
                        update_seq_num
                    );
                    Ok(())
                } else if book_seq_num + 1 == update_prev_seq_num {
                    log::debug!(
                        "our {} their prev {} their new {} - normal",
                        book_seq_num,
                        update_prev_seq_num,
                        update_seq_num
                    );
                    let _ = b.update(update);
                    Ok(())
                } else if update_prev_seq_num <= book_seq_num && book_seq_num < update_seq_num {
                    log::debug!(
                        "our {} their prev {} their new {} - catchup",
                        book_seq_num,
                        update_prev_seq_num,
                        update_seq_num
                    );
                    let _ = b.update(update);
                    Ok(())
                } else {
                    log::error!(
                        "our {} their prev {} their new {}",
                        book_seq_num,
                        update_prev_seq_num,
                        update_seq_num
                    );
                    Err("seq num mismatch".into())
                }
            }
        }
    }
}

// wss

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
        let WssData { E, s, U, u, b, a } = data;

        // bid < midprice, +ve quantity
        // ask > midprice, -ve quantity
        None
    }
}
