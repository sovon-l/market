pub fn de_ccy(s: &str) -> [u8; 6] {
    crate::util::symbol::str_to_asset(s)
}

const BASE_CURRENCIES: [&str; 1] = ["USDT"];

pub fn de_symbol(s: &str) -> Option<crate::structs::symbol::Symbol> {
    let (base, quote) = crate::util::symbol::split_currency_quote(s, &BASE_CURRENCIES)?;
    Some(crate::structs::symbol::Symbol {
        exchange: proper_market_api::Exchange::binance,
        base: de_ccy(base),
        quote: de_ccy(quote),
        symbol_type: crate::structs::symbol::SymbolType::Spot,
    })
}

// must use lower capital
pub fn se_ccy(a: &[u8; 6]) -> String {
    crate::util::symbol::asset_to_str(a).to_string()
}

pub fn se_symbol(s: &crate::structs::symbol::Symbol) -> String {
    se_ccy(&s.base) + &se_ccy(&s.quote)
}
