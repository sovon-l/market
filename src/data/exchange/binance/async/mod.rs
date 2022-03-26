pub mod bbo;
pub mod trade;

fn get_spot_bbo_links(i: &[&crate::structs::symbol::Symbol]) -> Vec<String> {
    vec![format!(
        "{}/stream?streams={}",
        *crate::env_var::MARKET_BINANCE_SPOT_WSS,
        i.iter()
            .map(|i| format!(
                "{}@bookTicker",
                crate::data::exchange::binance::serde::se_symbol(i)
            ))
            .collect::<Vec<String>>()
            .join("/")
    )]
}

fn get_spot_trade_links(i: &[&crate::structs::symbol::Symbol]) -> Vec<String> {
    vec![format!(
        "{}/stream?streams={}",
        *crate::env_var::MARKET_BINANCE_SPOT_WSS,
        i.iter()
            .map(|i| format!(
                "{}@trade",
                crate::data::exchange::binance::serde::se_symbol(i)
            ))
            .collect::<Vec<String>>()
            .join("/")
    )]
}

pub fn run(
    sender: crossbeam_channel::Sender<crate::message::Message>,
    instruments: &std::collections::HashSet<crate::structs::symbol::Symbol>,
) -> Vec<futures::future::BoxFuture<'static, ()>> {
    let mut rt = Vec::<futures::future::BoxFuture<'static, ()>>::new();

    let spot_instruments: Vec<&crate::structs::symbol::Symbol> = instruments
        .iter()
        .filter(|i| i.symbol_type == crate::structs::symbol::SymbolType::Spot)
        .collect();
    for bbo_url in get_spot_bbo_links(&spot_instruments).into_iter() {
        let sender_clone = sender.clone();

        let awork = crate::util::websocket::awork(bbo_url, bbo::wss(sender_clone), bbo::State);
        rt.push(Box::pin(awork));
    }
    for trade_url in get_spot_trade_links(&spot_instruments).into_iter() {
        let sender_clone = sender.clone();

        let awork = crate::util::websocket::awork(
            trade_url,
            trade::wss(sender_clone),
            trade::State::default(),
        );
        rt.push(Box::pin(awork));
    }
    rt
}
