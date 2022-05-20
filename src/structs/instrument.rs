
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Instrument {
    pub exchange: proper_market_api::Exchange,
    pub base: [u8; 6],
    pub quote: [u8; 6],
    pub instrument_type: InstrumentType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum InstrumentType {
    Spot,
    Future(Option<u32>),
}

impl std::fmt::Display for Instrument {
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
            match self.instrument_type {
                InstrumentType::Spot => "0".to_string(),
                InstrumentType::Future(expiry) => match expiry {
                    Some(expiry) => format!("1_{}", expiry),
                    None => "1".to_string(),
                },
            }
        )
    }
}

impl std::str::FromStr for Instrument {
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
        Ok(Instrument {
            exchange: match splits[0] {
                "binance" => proper_market_api::Exchange::binance,
                "ftx" => proper_market_api::Exchange::ftx,
                _ => return Err(()),
            },
            base: crate::util::symbol::str_to_asset(parts[0]),
            quote: crate::util::symbol::str_to_asset(parts[1]),
            instrument_type: if tokens.len() < 2 {
                InstrumentType::Spot
            } else {
                let splits: Vec<&str> = tokens[1].split('_').collect();
                match splits[0] {
                    "0" => InstrumentType::Spot,
                    "1" => InstrumentType::Future(if splits.len() < 2 {
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

pub fn encode_instrument<'a, T: proper_market_api::Writer<'a> + std::default::Default>(
    s: Instrument,
    instrument_e: &mut proper_market_api::InstrumentEncoder<T>,
) {
    let Instrument {
        exchange,
        quote,
        base,
        instrument_type,
    } = s;
    instrument_e.exchange(exchange);
    instrument_e.quote(quote);
    instrument_e.base(base);
    match instrument_type {
        InstrumentType::Spot => instrument_e.instrument_type(proper_market_api::instrument_type::InstrumentType::spot),
        InstrumentType::Future(expiry) => {
            instrument_e.instrument_type(proper_market_api::instrument_type::InstrumentType::future);
            if let Some(expiry) = expiry {
                instrument_e.expiry(expiry);
            }
        }
    }
}

pub fn decode_instrument<'a, T: proper_market_api::Reader<'a> + std::default::Default>(
    instrument_d: &mut proper_market_api::InstrumentDecoder<T>,
) -> Instrument {
    Instrument {
        exchange: instrument_d.exchange(),
        quote: instrument_d.quote(),
        base: instrument_d.base(),
        instrument_type: match instrument_d.instrument_type() {
            proper_market_api::instrument_type::InstrumentType::spot => InstrumentType::Spot,
            proper_market_api::instrument_type::InstrumentType::future => {
                InstrumentType::Future(instrument_d.expiry())
            }
            _ => panic!(),
        },
    }
}
