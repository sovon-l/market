pub struct MarketListener<S: MarketListenerTrait> {
    pub inner: S,
}

impl<S: MarketListenerTrait> messenger::subscriber::Listener<crate::message::Message>
    for MarketListener<S>
{
    fn on_msg(&mut self, msg: crate::message::Message) {
        match msg {
            crate::message::Message::BboMsg(m) => self.inner.on_bbo(m),
            crate::message::Message::TradesMsg(m) => self.inner.on_trades(m),
        }
    }
}

pub trait MarketListenerTrait {
    fn on_bbo(&mut self, msg: crate::structs::market_price::MarketPrice);
    fn on_trades(&mut self, msg: crate::structs::trades::Trades);
}

// below are implementation examples

// 1. zmqsbe subscriber
pub type ZmqSbeSubscriber = messenger::subscriber::ZmqSubscriber;

// 2. bus subscriber
pub type BusSubscriber = messenger::subscriber::BusSubscriber<crate::message::Message>;

// 3. crossbeam subscriber
pub type CrossbeamSubscriber = messenger::subscriber::CrossbeamSubscriber<crate::message::Message>;
