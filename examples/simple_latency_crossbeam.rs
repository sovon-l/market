fn main() {
    //     simple_logger::init_with_level(log::Level::Debug).unwrap();

    //     let (sender, receiver) = crossbeam_channel::unbounded();

    //     let btc = market::util::symbol::str_to_asset("btc");
    //     let eth = market::util::symbol::str_to_asset("eth");
    //     let usdt = market::util::symbol::str_to_asset("usdt");

    //     let instruments = vec![
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::binance,
    //             base: btc,
    //             quote: usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::binance,
    //             base: eth,
    //             quote: usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //         // market::structs::symbol::Symbol {
    //         //     exchange: proper_market_api::Exchange::ftx,
    //         //     base: proper_market_api::Asset::btc,
    //         //     quote: proper_market_api::Asset::usdt,
    //         //     symbol_type: market::structs::symbol::SymbolType::Spot,
    //         // },
    //         // market::structs::symbol::Symbol {
    //         //     exchange: proper_market_api::Exchange::ftx,
    //         //     base: proper_market_api::Asset::eth,
    //         //     quote: proper_market_api::Asset::usdt,
    //         //     symbol_type: market::structs::symbol::SymbolType::Spot,
    //         // },
    //         // market::structs::symbol::Symbol {
    //         //     exchange: proper_market_api::Exchange::ftx,
    //         //     base: proper_market_api::Asset::btc,
    //         //     quote: proper_market_api::Asset::usdt,
    //         //     symbol_type: market::structs::symbol::SymbolType::Future(None),
    //         // },
    //         // market::structs::symbol::Symbol {
    //         //     exchange: proper_market_api::Exchange::ftx,
    //         //     base: proper_market_api::Asset::eth,
    //         //     quote: proper_market_api::Asset::usdt,
    //         //     symbol_type: market::structs::symbol::SymbolType::Future(None),
    //         // },
    //     ];

    //     market::data::exchange::binance::sync::run(
    //         sender.clone(),
    //         instruments
    //             .iter()
    //             .filter(|i| i.exchange == proper_market_api::Exchange::binance)
    //             .map(|s| *s)
    //             .collect(),
    //     );
    //     // market::data::exchange::ftx::sync::run(
    //     //     sender.clone(),
    //     //     instruments
    //     //         .iter()
    //     //         .filter(|i| i.exchange == proper_market_api::Exchange::ftx)
    //     //         .map(|s| *s)
    //     //         .collect(),
    //     // );

    //     messenger::subscriber::subscriber_loop(
    //         receiver,
    //         market::api::market_listener::MarketListener {
    //             inner: LatencyListener {},
    //         },
    //     );
}

// pub struct LatencyListener;

// impl market::api::market_listener::MarketListenerTrait for LatencyListener {
//     fn on_bbo(&mut self, msg: market::structs::market_price::MarketPrice) {
//         let time_now = chrono::Utc::now().timestamp_nanos();
//         let time_msg = msg.market_timestamp;
//         let symbol = msg.symbol;
//         log::info!(
//             "{}",
//             serde_json::json!({
//                 "ts": time_msg,
//                 "latency_ms": (time_now - time_msg as i64) as f64 / 1_000_000.0,
//                 "symbol": symbol.to_string(),
//             })
//         );
//     }
//     fn on_trades(&mut self, msg: market::structs::trades::Trades) {
//         let time_now = chrono::Utc::now().timestamp_nanos();
//         let time_msg = msg.market_timestamp;
//         let symbol = msg.symbol;
//         log::info!(
//             "{}",
//             serde_json::json!({
//                 "ts": time_msg,
//                 "latency_ms": (time_now - time_msg as i64) as f64 / 1_000_000.0,
//                 "symbol": symbol.to_string(),
//             })
//         );
//     }
// }
