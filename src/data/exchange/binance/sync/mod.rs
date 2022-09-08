pub mod bbo;
pub mod trade;

fn get_spot_bbo_links(
    i: &[&proper_ma_structs::structs::market::instrument::Instrument],
) -> Vec<String> {
    vec![format!(
        "{}/stream?streams={}",
        *crate::env_var::MARKET_BINANCE_SPOT_WSS,
        i.iter()
            .map(|i| format!(
                "{}@bookTicker",
                crate::data::exchange::binance::serde::se_inst(i)
            ))
            .collect::<Vec<String>>()
            .join("/")
    )]
}

fn get_spot_trade_links(
    i: &[&proper_ma_structs::structs::market::instrument::Instrument],
) -> Vec<String> {
    vec![format!(
        "{}/stream?streams={}",
        *crate::env_var::MARKET_BINANCE_SPOT_WSS,
        i.iter()
            .map(|i| format!(
                "{}@trade",
                crate::data::exchange::binance::serde::se_inst(i)
            ))
            .collect::<Vec<String>>()
            .join("/")
    )]
}

pub fn run(
    sender: crossbeam_channel::Sender<crate::message::Message>,
    instruments: Vec<proper_ma_structs::structs::market::instrument::Instrument>,
) {
    let spot_instruments: Vec<&proper_ma_structs::structs::market::instrument::Instrument> =
        instruments
            .iter()
            .filter(|i| {
                i.instrument_type
                    == proper_ma_structs::structs::market::instrument::InstrumentType::Spot
            })
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
