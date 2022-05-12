lazy_static::lazy_static! {
    pub static ref MARKET_BINANCE_SPOT_WSS: String = std::env::var("MARKET_BINANCE_SPOT_WSS").unwrap_or_else(|_| String::from("wss://stream.binance.com:9443"));
    pub static ref MARKET_FTX_WSS: String = std::env::var("MARKET_FTX_WSS").unwrap_or_else(|_| String::from("wss://ftx.com/ws"));
}
