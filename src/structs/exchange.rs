
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Exchange {
    Binance,
    Ftx,
}

impl std::convert::From<proper_ma_api::Exchange> for Exchange {
    fn from(exchange: proper_ma_api::Exchange) -> Self {
        match exchange {
            proper_ma_api::Exchange::binance => Self::Binance,
            proper_ma_api::Exchange::ftx => Self::Ftx,
            _ => unimplemented!(),
        }
    }
}

impl std::convert::Into<proper_ma_api::Exchange> for Exchange {
    fn into(self) -> proper_ma_api::Exchange {
        match self {
            Self::Binance => proper_ma_api::Exchange::binance,
            Self::Ftx => proper_ma_api::Exchange::ftx,
        }
    }
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Binance => write!(f, "binance"),
            Self::Ftx => write!(f, "ftx"),
        }
    }
}

impl std::str::FromStr for Exchange {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "binance" => Ok(Self::Binance),
            "ftx" => Ok(Self::Ftx),
            _ => Err("Invalid exchange".into()),
        }
    }
}
