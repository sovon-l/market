pub fn de_ccy(s: &str) -> Option<proper_market_api::Asset> {
    match &s[..] {
        "BTC" => Some(proper_market_api::Asset::btc),
        "ETH" => Some(proper_market_api::Asset::eth),
        "USDT" => Some(proper_market_api::Asset::usdt),
        "USD" => Some(proper_market_api::Asset::usd),
        _ => None,
    }
}

pub fn de_symbol(s: &str) -> Option<crate::structs::symbol::Symbol> {
    let symbol = s;
    if symbol.find('-').is_some() {
        if symbol.find("move").is_some() {
            return None;
        }
        let v: Vec<&str> = symbol.split('-').collect();
        let (base, quote) = (v[0], v[1]);
        if quote == "PERP" {
            Some(crate::structs::symbol::Symbol {
                exchange: proper_market_api::Exchange::ftx,
                base: de_ccy(base).unwrap(),
                quote: proper_market_api::Asset::usd,
                symbol_type: crate::structs::symbol::SymbolType::Future(None),
            })
        } else if quote.chars().next().unwrap().is_digit(10) {
            Some(crate::structs::symbol::Symbol {
                exchange: proper_market_api::Exchange::ftx,
                base: de_ccy(base).unwrap(),
                quote: proper_market_api::Asset::usd,
                symbol_type: crate::structs::symbol::SymbolType::Future(Some(0)), // TODO: mmdd to unix epoch
            })
        } else {
            Some(crate::structs::symbol::Symbol {
                exchange: proper_market_api::Exchange::ftx,
                base: de_ccy(base).unwrap(),
                quote: de_ccy(quote).unwrap(),
                symbol_type: crate::structs::symbol::SymbolType::Future(None),
            })
        }
    } else {
        let v: Vec<&str> = symbol.split('/').collect();
        if v.len() < 2 {
            return None;
        }
        let (base, quote) = (v[0], v[1]);

        Some(crate::structs::symbol::Symbol {
            exchange: proper_market_api::Exchange::ftx,
            base: de_ccy(base).unwrap(),
            quote: de_ccy(quote).unwrap(),
            symbol_type: crate::structs::symbol::SymbolType::Spot,
        })
    }
}

pub fn se_ccy(a: &proper_market_api::Asset) -> &'static str {
    match a {
        proper_market_api::Asset::usd => "usd",
        proper_market_api::Asset::btc => "btc",
        proper_market_api::Asset::usdt => "usdt",
        proper_market_api::Asset::eth => "eth",
        c => panic!("unexpect serializing uncovered ccy {:?}", c),
    }
}

pub fn se_symbol(s: &crate::structs::symbol::Symbol) -> String {
    match s.symbol_type {
        crate::structs::symbol::SymbolType::Spot => {
            format!("{}/{}", se_ccy(&s.base), se_ccy(&s.quote))
        }
        crate::structs::symbol::SymbolType::Future(expiry) => {
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
