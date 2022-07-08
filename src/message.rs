#[derive(Debug, Clone)]
pub enum Message {
    QuotesMsg(proper_ma_structs::structs::market::quotes::Quotes),
    TradesMsg(proper_ma_structs::structs::market::trades::Trades),
}

impl From<Message> for zmq::Message {
    fn from(s: Message) -> zmq::Message {
        match s {
            Message::QuotesMsg(msg) => proper_ma_structs::sbe::market::quotes_msg::marshal_quotes_msg(msg).into(),
            Message::TradesMsg(msg) => proper_ma_structs::sbe::market::trades_msg::marshal_trades_msg(msg).into(),
        }
    }
}

impl From<Vec<u8>> for Message {
    fn from(item: Vec<u8>) -> Self {
        decode_message(&item)
    }
}

pub fn decode_message(v: &[u8]) -> Message {
    let buf = proper_ma_api::ReadBuf::new(v);
    let header = proper_ma_api::MessageHeaderDecoder::default().wrap(buf, 0);
    let template_id = header.template_id();
    let schema_id = header.schema_id();
    let version = header.version();

    // TODO: decode should pass &[u8] without header?
    match (schema_id, version, template_id) {
        (1, 1, 1) => Message::QuotesMsg(proper_ma_structs::sbe::market::quotes_msg::unmarshal_quotes_msg(v)),
        (1, 1, 2) => Message::TradesMsg(proper_ma_structs::sbe::market::trades_msg::unmarshal_trades_msg(v)),
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
