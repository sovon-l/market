#[derive(Debug, Clone)]
pub struct Trades {
    pub symbol: crate::structs::instrument::Instrument,
    pub market_timestamp: u64,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub price: rust_decimal::Decimal,
    pub size: rust_decimal::Decimal,
    pub timestamp: u64,
    // tradeId: u32,
}

impl From<Trades> for zmq::Message {
    fn from(s: Trades) -> zmq::Message {
        let mut buffer = vec![
            0u8;
            proper_market_api::message_header_codec::ENCODED_LENGTH
                + proper_market_api::trade_msg_codec::SBE_BLOCK_LENGTH as usize
                + proper_market_api::trade_msg_codec::TradesEncoder::<
                    proper_market_api::trade_msg_codec::TradeMsgEncoder,
                >::block_length() as usize
                    * s.trades.len()
                + 3
        ];
        encode_trades(&mut buffer, s);
        zmq::Message::from(buffer)
    }
}

pub fn encode_trades(buffer: &mut [u8], ts: Trades) {
    let Trades {
        symbol,
        market_timestamp,
        trades,
    } = ts;

    let mut trades_msg = proper_market_api::TradeMsgEncoder::default();
    trades_msg = trades_msg.wrap(
        proper_market_api::WriteBuf::new(buffer),
        proper_market_api::message_header_codec::ENCODED_LENGTH,
    );
    trades_msg = trades_msg.header(0).parent().unwrap();
    let mut symbol_e = trades_msg.instrument_encoder();
    crate::structs::instrument::encode_instrument(symbol, &mut symbol_e);
    trades_msg = symbol_e.parent().unwrap();

    trades_msg.market_timestamp(market_timestamp);

    let mut trades_e = proper_market_api::TradesEncoder::default();
    trades_e = trades_msg.trades_encoder(trades.len() as u8, trades_e);
    for Trade {
        price,
        size,
        timestamp,
    } in trades.into_iter()
    {
        trades_e.advance().unwrap();

        let mut price_e = trades_e.price_encoder();
        price_e.mantissa(price.mantissa() as i64);
        price_e.exponent(price.scale() as i8);
        trades_e = price_e.parent().unwrap();

        let mut size_e = trades_e.size_encoder();
        size_e.mantissa(size.mantissa() as i64);
        size_e.exponent(size.scale() as i8);
        trades_e = size_e.parent().unwrap();

        trades_e.timestamp(timestamp);
    }
    // trades_msg = trades_e.parent().unwrap();
}

pub fn decode_trades(v: &[u8]) -> Trades {
    let mut trades_msg_d = proper_market_api::TradeMsgDecoder::default();
    let buf = proper_market_api::ReadBuf::new(v);
    let header = proper_market_api::MessageHeaderDecoder::default().wrap(buf, 0);
    trades_msg_d = trades_msg_d.header(header);

    let mut symbol_d = trades_msg_d.instrument_decoder();
    let symbol = crate::structs::instrument::decode_instrument(&mut symbol_d);
    trades_msg_d = symbol_d.parent().unwrap();

    let market_timestamp = trades_msg_d.market_timestamp();

    let mut trades = vec![];
    let mut trades_d = trades_msg_d.trades_decoder();
    while let Ok(Some(_)) = trades_d.advance() {
        let mut trade_price_d = trades_d.price_decoder();
        let price =
            rust_decimal::Decimal::new(trade_price_d.mantissa(), trade_price_d.exponent() as u32);
        trades_d = trade_price_d.parent().unwrap();

        let mut trade_size_d = trades_d.size_decoder();
        let size =
            rust_decimal::Decimal::new(trade_size_d.mantissa(), trade_size_d.exponent() as u32);
        trades_d = trade_size_d.parent().unwrap();

        trades.push(Trade {
            price,
            size,
            timestamp: trades_d.timestamp(),
        });
    }

    Trades {
        symbol,
        market_timestamp,
        trades,
    }
}
