#[derive(Debug, Clone)]
pub struct MarketPrice {
    pub symbol: crate::structs::instrument::Instrument,
    pub market_timestamp: u64,
    pub timestamp: Option<u64>, // ns
    pub bid_price: rust_decimal::Decimal,
    pub bid_size: rust_decimal::Decimal,
    pub ask_price: rust_decimal::Decimal,
    pub ask_size: rust_decimal::Decimal,
}

impl From<MarketPrice> for zmq::Message {
    fn from(s: MarketPrice) -> zmq::Message {
        let mut buffer = vec![
            0u8;
            proper_ma_api::bbo_msg_codec::SBE_BLOCK_LENGTH as usize
                + proper_ma_api::message_header_codec::ENCODED_LENGTH
        ];
        encode_market_price(&mut buffer, s);
        zmq::Message::from(buffer)
    }
}

pub fn encode_market_price(buffer: &mut [u8], mp: MarketPrice) {
    let MarketPrice {
        symbol,
        market_timestamp,
        timestamp,
        bid_price,
        bid_size,
        ask_price,
        ask_size,
    } = mp;

    let mut bbo_msg = proper_ma_api::BboMsgEncoder::default();
    bbo_msg = bbo_msg.wrap(
        proper_ma_api::WriteBuf::new(buffer),
        proper_ma_api::message_header_codec::ENCODED_LENGTH,
    );
    bbo_msg = bbo_msg.header(0).parent().unwrap();
    let mut symbol_e = bbo_msg.instrument_encoder();
    crate::structs::instrument::encode_instrument(symbol, &mut symbol_e);
    bbo_msg = symbol_e.parent().unwrap();

    bbo_msg.market_timestamp(market_timestamp);

    if let Some(timestamp) = timestamp {
        bbo_msg.timestamp(timestamp);
    }

    let mut bid_price_e = bbo_msg.bid_price_encoder();
    bid_price_e.mantissa(bid_price.mantissa() as i64);
    bid_price_e.exponent(bid_price.scale() as i8);
    bbo_msg = bid_price_e.parent().unwrap();

    let mut bid_size_e = bbo_msg.bid_size_encoder();
    bid_size_e.mantissa(bid_size.mantissa() as i64);
    bid_size_e.exponent(bid_size.scale() as i8);
    bbo_msg = bid_size_e.parent().unwrap();

    let mut ask_price_e = bbo_msg.ask_price_encoder();
    ask_price_e.mantissa(ask_price.mantissa() as i64);
    ask_price_e.exponent(ask_price.scale() as i8);
    bbo_msg = ask_price_e.parent().unwrap();

    let mut ask_size_e = bbo_msg.ask_size_encoder();
    ask_size_e.mantissa(ask_size.mantissa() as i64);
    ask_size_e.exponent(ask_size.scale() as i8);
    // bbo_msg = ask_size_e.parent().unwrap();

    // let length = bbo_msg.encoded_length() + proper_ma_api::message_header_codec::ENCODED_LENGTH;
    // buffer.iter().take(length).map(|b| *b).collect()
}

pub fn decode_market_price(v: &[u8]) -> MarketPrice {
    let mut bbo_msg_d = proper_ma_api::BboMsgDecoder::default();
    let buf = proper_ma_api::ReadBuf::new(v);
    let header = proper_ma_api::MessageHeaderDecoder::default().wrap(buf, 0);
    bbo_msg_d = bbo_msg_d.header(header);

    let mut symbol_d = bbo_msg_d.instrument_decoder();
    let symbol = crate::structs::instrument::decode_instrument(&mut symbol_d);
    bbo_msg_d = symbol_d.parent().unwrap();

    let mut bid_price_d = bbo_msg_d.bid_price_decoder();
    let bid_price =
        rust_decimal::Decimal::new(bid_price_d.mantissa(), bid_price_d.exponent() as u32);
    bbo_msg_d = bid_price_d.parent().unwrap();

    let mut bid_size_d = bbo_msg_d.bid_size_decoder();
    let bid_size = rust_decimal::Decimal::new(bid_size_d.mantissa(), bid_size_d.exponent() as u32);
    bbo_msg_d = bid_size_d.parent().unwrap();

    let mut ask_price_d = bbo_msg_d.ask_price_decoder();
    let ask_price =
        rust_decimal::Decimal::new(ask_price_d.mantissa(), ask_price_d.exponent() as u32);
    bbo_msg_d = ask_price_d.parent().unwrap();

    let mut ask_size_d = bbo_msg_d.ask_size_decoder();
    let ask_size = rust_decimal::Decimal::new(ask_size_d.mantissa(), ask_size_d.exponent() as u32);
    bbo_msg_d = ask_size_d.parent().unwrap();

    MarketPrice {
        symbol,
        market_timestamp: bbo_msg_d.market_timestamp(),
        timestamp: bbo_msg_d.timestamp(),
        bid_price,
        bid_size,
        ask_price,
        ask_size,
    }
}
