#[derive(Debug, Clone)]
pub struct Depth {
    pub price: rust_decimal::Decimal,
    pub size: rust_decimal::Decimal,
}

#[derive(Debug, Clone)]
pub struct Quotes {
    pub symbol: crate::structs::instrument::Instrument,
    pub market_timestamp: u64,
    pub timestamp: Option<u64>, // ns
    pub is_snapshot: bool,
    pub is_l1: bool,
    pub depths: Vec<Depth>,
}

impl From<Quotes> for zmq::Message {
    fn from(s: Quotes) -> zmq::Message {
        let mut buffer = vec![
            0u8;
            proper_ma_api::message_header_codec::ENCODED_LENGTH
                + proper_ma_api::quote_msg_codec::SBE_BLOCK_LENGTH as usize
                + proper_ma_api::quote_msg_codec::DepthsEncoder::<
                    proper_ma_api::quote_msg_codec::QuoteMsgEncoder,
                >::block_length() as usize
                    * s.depths.len()
                + 3
        ];
        encode_quotes(&mut buffer, s);
        zmq::Message::from(buffer)
    }
}

pub fn encode_quotes(buffer: &mut [u8], q: Quotes) {
    let Quotes {
        symbol,
        market_timestamp,
        timestamp,
        is_snapshot,
        is_l1,
        depths,
    } = q;

    let mut quotes_msg = proper_ma_api::QuoteMsgEncoder::default();
    quotes_msg = quotes_msg.wrap(
        proper_ma_api::WriteBuf::new(buffer),
        proper_ma_api::message_header_codec::ENCODED_LENGTH,
    );
    quotes_msg = quotes_msg.header(0).parent().unwrap();
    let mut symbol_e = quotes_msg.instrument_encoder();
    crate::structs::instrument::encode_instrument(symbol, &mut symbol_e);
    quotes_msg = symbol_e.parent().unwrap();

    quotes_msg.market_timestamp(market_timestamp);

    if let Some(timestamp) = timestamp {
        quotes_msg.timestamp(timestamp);
    }

    let mut orderbook_flags_e = proper_ma_api::OrderbookFlags::new(0);
    orderbook_flags_e.set_is_snapshot(is_snapshot);
    orderbook_flags_e.set_l1(is_l1);
    quotes_msg.orderbook_flags(orderbook_flags_e);

    let mut depths_e = proper_ma_api::DepthsEncoder::default();
    depths_e = quotes_msg.depths_encoder(depths.len() as u8, depths_e);
    for Depth { price, size } in depths.into_iter() {
        depths_e.advance().unwrap();

        let mut price_e = depths_e.price_encoder();
        price_e.mantissa(price.mantissa() as i64);
        price_e.exponent(price.scale() as i8);
        depths_e = price_e.parent().unwrap();

        let mut size_e = depths_e.size_encoder();
        size_e.mantissa(size.mantissa() as i64);
        size_e.exponent(size.scale() as i8);
        depths_e = size_e.parent().unwrap();
    }
}

pub fn decode_quotes(v: &[u8]) -> Quotes {
    let mut quotes_msg_d = proper_ma_api::QuoteMsgDecoder::default();
    let buf = proper_ma_api::ReadBuf::new(v);
    let header = proper_ma_api::MessageHeaderDecoder::default().wrap(buf, 0);
    quotes_msg_d = quotes_msg_d.header(header);

    let mut symbol_d = quotes_msg_d.instrument_decoder();
    let symbol = crate::structs::instrument::decode_instrument(&mut symbol_d);
    quotes_msg_d = symbol_d.parent().unwrap();

    let market_timestamp = quotes_msg_d.market_timestamp();

    let timestamp = quotes_msg_d.timestamp();

    let orderbook_flags_d = quotes_msg_d.orderbook_flags();
    let is_snapshot = orderbook_flags_d.get_is_snapshot();
    let is_l1 = orderbook_flags_d.get_l1();

    let mut depths_d = quotes_msg_d.depths_decoder();
    let depths_count = depths_d.count();
    let mut depths = Vec::with_capacity(depths_count as usize);
    for _ in 0..depths_count {
        depths_d.advance().unwrap();

        let mut price_d = depths_d.price_decoder();
        let price = rust_decimal::Decimal::new(
            price_d.mantissa(),
            price_d.exponent() as u32,
        );
        depths_d = price_d.parent().unwrap();

        let mut size_d = depths_d.size_decoder();
        let size = rust_decimal::Decimal::new(
            size_d.mantissa(),
            size_d.exponent() as u32,
        );
        depths_d = size_d.parent().unwrap();

        depths.push(Depth { price, size });
    }
    
    Quotes {
        symbol,
        market_timestamp,
        timestamp,
        is_snapshot,
        is_l1,
        depths,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn encode() {
        let test_data = vec![
            Quotes {
                symbol: crate::structs::instrument::Instrument::from_str("binance:btc_usdt").unwrap(),
                market_timestamp: 123456789,
                timestamp: Some(123456789),
                is_snapshot: true,
                is_l1: true,
                depths: vec![
                    Depth {
                        price: rust_decimal::Decimal::new(20000, 0),
                        size: rust_decimal::Decimal::new(-1, 0),
                    },
                    Depth {
                        price: rust_decimal::Decimal::new(21000, 0),
                        size: rust_decimal::Decimal::new(1, 0),
                    },
                ],
            },
            Quotes {
                symbol: crate::structs::instrument::Instrument::from_str("binance:eth_usdt").unwrap(),
                market_timestamp: 123456789,
                timestamp: Some(123456789),
                is_snapshot: true,
                is_l1: false,
                depths: {
                    let mut depths = vec![];
                    let p = rust_decimal::Decimal::new(20000, 0);
                    let s = rust_decimal::Decimal::new(-1, 0);
                    for i in 0..10 {
                        depths.push(Depth {
                            price: p + rust_decimal::Decimal::from(i),
                            size: s + rust_decimal::Decimal::from(2 * i),
                        });
                    }
                    depths
                },
            }
        ];

        let test_data = test_data.into_iter().map(|d| zmq::Message::from(d)).collect::<Vec<_>>();

        let test_data = test_data.into_iter().map(|u| {
            if let crate::message::Message::QuotesMsg(m) = crate::message::decode_message(&u) {
                m
            } else {
                panic!("decode_message failed");
            }
        }).collect::<Vec<_>>();

        assert_eq!(test_data[1].depths[1].price, rust_decimal::Decimal::new(20001, 0));
    }
}