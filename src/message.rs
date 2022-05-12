#[derive(Debug, Clone)]
pub enum Message {
    BboMsg(crate::structs::market_price::MarketPrice),
    TradesMsg(crate::structs::trades::Trades),
}

impl From<Message> for zmq::Message {
    fn from(s: Message) -> zmq::Message {
        match s {
            Message::BboMsg(msg) => msg.into(),
            Message::TradesMsg(msg) => msg.into(),
        }
    }
}

impl From<Vec<u8>> for Message {
    fn from(item: Vec<u8>) -> Self {
        decode_message(&item)
    }
}

pub fn decode_message(v: &[u8]) -> Message {
    let buf = proper_market_api::ReadBuf::new(v);
    let header = proper_market_api::MessageHeaderDecoder::default().wrap(buf, 0);
    let template_id = header.template_id();
    let schema_id = header.schema_id();
    let version = header.version();

    // TODO: decode should pass &[u8] without header?
    match (schema_id, version, template_id) {
        (1, 1, 1) => Message::BboMsg(crate::structs::market_price::decode_market_price(v)),
        (1, 1, 2) => Message::TradesMsg(crate::structs::trades::decode_trades(v)),
        (s, v, t) => {
            unimplemented!(
                "cannot decode msg: schema({}) version({}) template_id({}",
                s,
                v,
                t
            );
        }
    }
}
