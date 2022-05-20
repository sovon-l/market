pub fn de_ccy(s: &str) -> [u8; 6] {
    crate::util::symbol::str_to_asset(s)
}

const BASE_CURRENCIES: [&'static str; 1] = ["USDT"];

pub fn de_inst(s: &str) -> Option<crate::structs::instrument::Instrument> {
    let (base, quote) = crate::util::symbol::split_currency_quote(s, &BASE_CURRENCIES)?;
    Some(crate::structs::instrument::Instrument {
        exchange: proper_market_api::Exchange::binance,
        base: de_ccy(base),
        quote: de_ccy(quote),
        instrument_type: crate::structs::instrument::InstrumentType::Spot,
    })
}

pub fn se_ccy(a: &[u8; 6]) -> String {
    let v = crate::util::symbol::asset_to_str(a);
    v.to_uppercase()
}

pub fn se_inst(s: &crate::structs::instrument::Instrument) -> String {
    se_ccy(&s.base) + &se_ccy(&s.quote)
}
