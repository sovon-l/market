pub fn de_ccy(s: &str) -> [u8; 6] {
    crate::util::symbol::str_to_asset(s)
}

const BASE_CURRENCIES: [&str; 1] = ["USDT"];

pub fn de_inst(s: &str) -> Option<crate::structs::instrument::Instrument> {
    let (base, quote) = crate::util::symbol::split_currency_quote(s, &BASE_CURRENCIES)?;
    Some(crate::structs::instrument::Instrument {
        exchange: crate::structs::exchange::Exchange::Binance,
        base: de_ccy(base),
        quote: de_ccy(quote),
        instrument_type: crate::structs::instrument::InstrumentType::Spot,
    })
}

// must use lower capital
pub fn se_ccy(a: &[u8; 6]) -> String {
    crate::util::symbol::asset_to_str(a).to_string()
}

pub fn se_inst(s: &crate::structs::instrument::Instrument) -> String {
    se_ccy(&s.base) + &se_ccy(&s.quote)
}
