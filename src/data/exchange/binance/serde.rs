pub fn de_ccy(s: &str) -> [u8; 6] {
    crate::util::symbol::str_to_asset(s)
}

const BASE_CURRENCIES: [&str; 1] = ["USDT"];

pub fn de_inst(s: &str) -> Option<proper_ma_structs::structs::market::instrument::Instrument> {
    let (base, quote) = crate::util::symbol::split_currency_quote(s, &BASE_CURRENCIES)?;
    Some(proper_ma_structs::structs::market::instrument::Instrument {
        exchange: proper_ma_structs::structs::market::exchange::Exchange::Binance,
        base: de_ccy(base),
        quote: de_ccy(quote),
        instrument_type: proper_ma_structs::structs::market::instrument::InstrumentType::Spot,
    })
}

// must use lower capital
pub fn se_ccy(a: &[u8; 6]) -> String {
    crate::util::symbol::asset_to_str(a).to_string()
}

pub fn se_inst(s: &proper_ma_structs::structs::market::instrument::Instrument) -> String {
    se_ccy(&s.base) + &se_ccy(&s.quote)
}
