use crate::data::orderbook::{MbpFullbook, MbpUpdate, VerificationStatus};
use proper_ma_structs::structs::market::instrument::Instrument;
use std::error::Error;

pub trait MbpCache<VU, VS: VerificationStatus<VerificationUpdate = VU>>: Send + Sync {
    fn is_book_built(&self) -> bool;
    fn new_response(book: MbpFullbook<VS>) -> Self
    where
        Self: Sized;
    fn new_update(update: MbpUpdate<VU>) -> Self
    where
        Self: Sized;
    fn process_response(&self, book: MbpFullbook<VS>) -> Result<Option<Self>, Box<dyn Error>>
    where
        Self: Sized;
    fn process_update(&mut self, update: MbpUpdate<VU>) -> Result<(), Box<dyn Error>>;
}

pub trait MbpHandler<VU, VS: VerificationStatus<VerificationUpdate = VU>, T: MbpCache<VU, VS>>:
    Send + Sync
{
    // fn request_for_snapshot(&self, inst: &Instrument) -> Result<(), Box<dyn Error>>;
}
