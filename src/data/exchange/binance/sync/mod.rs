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
    instruments: Vec<crate::structs::symbol::Symbol>,
) {
    let spot_instruments: Vec<&crate::structs::symbol::Symbol> = instruments
        .iter()
        .filter(|i| i.symbol_type == crate::structs::symbol::SymbolType::Spot)
        .collect();
    for bbo_url in get_spot_bbo_links(&spot_instruments) {
        let sender_clone = sender.clone();
        std::thread::Builder::new()
            .name("binance trade".into())
            .spawn(move || {
                crate::util::websocket::work(&bbo_url, bbo::wss(sender_clone), bbo::State {})
            })
            .unwrap();
    }
    for trade_url in get_spot_trade_links(&spot_instruments) {
        let sender_clone = sender.clone();
        std::thread::Builder::new()
            .name("binance bbo".into())
            .spawn(move || {
                crate::util::websocket::work(
                    &trade_url,
                    trade::wss(sender_clone),
                    trade::State::default(),
                )
            })
            .unwrap();
    }
}
