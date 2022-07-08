pub struct MarketListener<S: MarketListenerTrait> {
    pub inner: S,
}

impl<S: MarketListenerTrait> messenger::pubsub::Listener<crate::message::Message>
    for MarketListener<S>
{
    fn on_msg(&mut self, msg: crate::message::Message) {
        match msg {
            crate::message::Message::QuotesMsg(m) => self.inner.on_quotes(m),
            crate::message::Message::TradesMsg(m) => self.inner.on_trades(m),
        }
    }
}

pub trait MarketListenerTrait {
    fn on_quotes(&mut self, msg: proper_ma_structs::structs::market::quotes::Quotes);
    fn on_trades(&mut self, msg: proper_ma_structs::structs::market::trades::Trades);
}
