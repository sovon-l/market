fn main() {
    //     simple_logger::init_with_level(log::Level::Info).unwrap();

    //     let m = std::sync::Arc::new(std::sync::Mutex::new(zmq::Context::new()));

    //     let (sender, receiver) = crossbeam_channel::unbounded();
    //     let m_clone = m.clone();
    //     let publisher = std::thread::spawn(move || {
    //         market::data::publisher::publisher(m_clone, receiver, |s| {
    //             s.bind("tcp://*:8000").unwrap();
    //         });
    //     });

    //     let instruments = vec![
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::binance,
    //             base: proper_market_api::Asset::btc,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::ftx,
    //             base: proper_market_api::Asset::btc,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //     ];

    //     market::data::exchange::binance::sync::run(
    //         sender.clone(),
    //         instruments
    //             .iter()
    //             .filter(|i| i.exchange == proper_market_api::Exchange::binance)
    //             .map(|s| *s)
    //             .collect(),
    //     );
    //     market::data::exchange::ftx::sync::run(
    //         sender.clone(),
    //         instruments
    //             .iter()
    //             .filter(|i| i.exchange == proper_market_api::Exchange::ftx)
    //             .map(|s| *s)
    //             .collect(),
    //     );

    //     let _ = publisher.join();
}
