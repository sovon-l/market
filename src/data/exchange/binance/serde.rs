pub fn de_ccy(s: &str) -> Option<proper_market_api::Asset> {
    match &s[..] {
        "BTC" => Some(proper_market_api::Asset::btc),
        "ETH" => Some(proper_market_api::Asset::eth),
        "USDT" => Some(proper_market_api::Asset::usdt),
        "USD" => Some(proper_market_api::Asset::usd),
        _ => None,
    }
}

const BASE_CURRENCIES: [&'static str; 1] = ["USDT"];

pub fn de_symbol(s: &str) -> Option<crate::structs::symbol::Symbol> {
    let (base, quote) = crate::util::symbol::split_currency_quote(s, &BASE_CURRENCIES)?;
    Some(crate::structs::symbol::Symbol {
        exchange: proper_market_api::Exchange::binance,
        base: de_ccy(base)?,
        quote: de_ccy(quote)?,
        symbol_type: crate::structs::symbol::SymbolType::Spot,
    })
}

pub fn se_ccy(a: &proper_market_api::Asset) -> &'static str {
    match a {
        proper_market_api::Asset::usd => "usd",
        proper_market_api::Asset::btc => "btc",
        proper_market_api::Asset::usdt => "usdt",
        proper_market_api::Asset::eth => "eth",
        c => panic!("unexpect serializing uncovered {:?}", c),
    }
}

pub fn se_symbol(s: &crate::structs::symbol::Symbol) -> String {
    se_ccy(&s.base).to_owned() + se_ccy(&s.quote)
}
