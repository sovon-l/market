lazy_static::lazy_static! {
    pub static ref MARKET_BINANCE_SPOT_WSS: String = std::env::var("MARKET_BINANCE_SPOT_WSS").unwrap_or("wss://stream.binance.com:9443".to_string());
    pub static ref MARKET_FTX_WSS: String = std::env::var("MARKET_FTX_WSS").unwrap_or("wss://ftx.com/ws".to_string());
}
