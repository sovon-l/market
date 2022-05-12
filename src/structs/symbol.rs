
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Symbol {
    pub exchange: proper_market_api::Exchange,
    pub base: [u8; 6],
    pub quote: [u8; 6],
    pub symbol_type: SymbolType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum SymbolType {
    Spot,
    Future(Option<u32>),
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}_{}-{}",
            match self.exchange {
                proper_market_api::Exchange::binance => "binance",
                proper_market_api::Exchange::ftx => "ftx",
                _ => panic!(),
            },
            std::str::from_utf8(&self.base).unwrap(),
            std::str::from_utf8(&self.quote).unwrap(),
            match self.symbol_type {
                SymbolType::Spot => "0".to_string(),
                SymbolType::Future(expiry) => match expiry {
                    Some(expiry) => format!("1_{}", expiry),
                    None => "1".to_string(),
                },
            }
        )
    }
}

impl std::str::FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(':').collect();
        if splits.len() < 2 {
            return Err(());
        }
        let tokens: Vec<&str> = splits[1].split('-').collect();
        let parts: Vec<&str> = tokens[0].split('_').collect();
        if parts.len() < 2 {
            return Err(());
        }
        Ok(Symbol {
            exchange: match splits[0] {
                "binance" => proper_market_api::Exchange::binance,
                "ftx" => proper_market_api::Exchange::ftx,
                _ => return Err(()),
            },
            base: crate::util::symbol::str_to_asset(parts[0]),
            quote: crate::util::symbol::str_to_asset(parts[1]),
            symbol_type: if tokens.len() < 2 {
                SymbolType::Spot
            } else {
                let splits: Vec<&str> = tokens[1].split('_').collect();
                match splits[0] {
                    "0" => SymbolType::Spot,
                    "1" => SymbolType::Future(if splits.len() < 2 {
                        None
                    } else {
                        Some(if let Ok(v) = splits[1].parse() {
                            v
                        } else {
                            return Err(());
                        })
                    }),
                    _ => return Err(()),
                }
            },
        })
    }
}

pub fn encode_symbol<'a, T: proper_market_api::Writer<'a> + std::default::Default>(
    s: Symbol,
    symbol_e: &mut proper_market_api::SymbolEncoder<T>,
) {
    let Symbol {
        exchange,
        quote,
        base,
        symbol_type,
    } = s;
    symbol_e.exchange(exchange);
    symbol_e.quote(quote);
    symbol_e.base(base);
    match symbol_type {
        SymbolType::Spot => symbol_e.symbol_type(proper_market_api::symbol_type::SymbolType::spot),
        SymbolType::Future(expiry) => {
            symbol_e.symbol_type(proper_market_api::symbol_type::SymbolType::future);
            if let Some(expiry) = expiry {
                symbol_e.expiry(expiry);
            }
        }
    }
}

pub fn decode_symbol<'a, T: proper_market_api::Reader<'a> + std::default::Default>(
    symbol_d: &mut proper_market_api::SymbolDecoder<T>,
) -> Symbol {
    Symbol {
        exchange: symbol_d.exchange(),
        quote: symbol_d.quote(),
        base: symbol_d.base(),
        symbol_type: match symbol_d.symbol_type() {
            proper_market_api::symbol_type::SymbolType::spot => SymbolType::Spot,
            proper_market_api::symbol_type::SymbolType::future => {
                SymbolType::Future(symbol_d.expiry())
            }
            _ => panic!(),
        },
    }
}
