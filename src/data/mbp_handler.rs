
use proper_ma_structs::structs::market::instrument::Instrument;
use std::error::Error;


pub trait MbpBuilder: Send + Sync {
    // fn is_book_built(&self) -> bool;
    // fn get_book(&self) -> Result<&MbpOrderbook, Box<dyn Error>>;


}


pub trait MbpHandler<T: MbpBuilder>: Send + Sync {
    // fn request_for_snapshot(&self, inst: &Instrument) -> Result<(), Box<dyn Error>>;

}
