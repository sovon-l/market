pub fn de_ccy(s: &str) -> [u8; 6] {
    crate::util::symbol::str_to_asset(s)
}

pub fn de_inst(s: &str) -> Option<crate::structs::instrument::Instrument> {
    let symbol = s;
    if symbol.find('-').is_some() {
        if symbol.contains("move") {
            return None;
        }
        let v: Vec<&str> = symbol.split('-').collect();
        let (base, quote) = (v[0], v[1]);
        if quote == "PERP" {
            Some(crate::structs::instrument::Instrument {
                exchange: crate::structs::exchange::Exchange::Ftx,
                base: de_ccy(base),
                quote: de_ccy("usd"),
                instrument_type: crate::structs::instrument::InstrumentType::Future(None),
            })
        } else if quote.chars().next().unwrap().is_digit(10) {
            Some(crate::structs::instrument::Instrument {
                exchange: crate::structs::exchange::Exchange::Ftx,
                base: de_ccy(base),
                quote: de_ccy("usd"),
                instrument_type: crate::structs::instrument::InstrumentType::Future(Some(0)), // TODO: mmdd to unix epoch
            })
        } else {
            Some(crate::structs::instrument::Instrument {
                exchange: crate::structs::exchange::Exchange::Ftx,
                base: de_ccy(base),
                quote: de_ccy(quote),
                instrument_type: crate::structs::instrument::InstrumentType::Future(None),
            })
        }
    } else {
        let v: Vec<&str> = symbol.split('/').collect();
        if v.len() < 2 {
            return None;
        }
        let (base, quote) = (v[0], v[1]);

        Some(crate::structs::instrument::Instrument {
            exchange: crate::structs::exchange::Exchange::Ftx,
            base: de_ccy(base),
            quote: de_ccy(quote),
            instrument_type: crate::structs::instrument::InstrumentType::Spot,
        })
    }
}

pub fn se_ccy(a: &[u8; 6]) -> String {
    crate::util::symbol::asset_to_str(a).to_owned()
}

pub fn se_inst(s: &crate::structs::instrument::Instrument) -> String {
    match s.instrument_type {
        crate::structs::instrument::InstrumentType::Spot => {
            format!("{}/{}", se_ccy(&s.base), se_ccy(&s.quote))
        }
        crate::structs::instrument::InstrumentType::Future(expiry) => {
            if let Some(expiry) = expiry {
                format!(
                    "{}-{}",
                    se_ccy(&s.base),
                    chrono::NaiveDateTime::from_timestamp(expiry as i64, 0).format("%m%d")
                )
            } else {
                format!("{}-PERP", se_ccy(&s.base))
            }
        } // s => panic!("unexpect serializing uncovered symbol type {:?}", s),
    }
}
