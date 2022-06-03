pub fn from_str(s: &str) -> Result<proper_market_api::Exchange, Box<dyn std::error::Error>> {
    let exchange = match &s[..] {
        "binance" => proper_market_api::Exchange::binance,
        "ftx" => proper_market_api::Exchange::ftx,
        _ => return Err("unknown exchange".into()),
    };
    Ok(exchange)
}
