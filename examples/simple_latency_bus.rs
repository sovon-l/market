fn main() {
    //     simple_logger::init_with_level(log::Level::Info).unwrap();

    //     let (sender, receiver) = crossbeam_channel::unbounded();

    //     let instruments = vec![
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::binance,
    //             base: proper_market_api::Asset::btc,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::binance,
    //             base: proper_market_api::Asset::eth,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::ftx,
    //             base: proper_market_api::Asset::btc,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::ftx,
    //             base: proper_market_api::Asset::eth,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Spot,
    //         },
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::ftx,
    //             base: proper_market_api::Asset::btc,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Future(None),
    //         },
    //         market::structs::symbol::Symbol {
    //             exchange: proper_market_api::Exchange::ftx,
    //             base: proper_market_api::Asset::eth,
    //             quote: proper_market_api::Asset::usdt,
    //             symbol_type: market::structs::symbol::SymbolType::Future(None),
    //         },
    //     ];

    //     let controller = market::data::controller::work(
    //         sender,
    //         std::sync::Arc::new(tokio::sync::Mutex::new(
    //             market::data::controller::State::default(),
    //         )),
    //         instruments,
    //     );

    //     let mut b = bus::Bus::new(100);
    //     let br = b.add_rx();

    //     let _ = std::thread::spawn(move || {
    //         messenger::publisher::publisher_loop(receiver, b);
    //     });

    //     let _ = std::thread::spawn(move || {
    //         messenger::subscriber::subscriber_loop(
    //             br,
    //             market::api::market_listener::MarketListener {
    //                 inner: LatencyListener {},
    //             },
    //         );
    //     });

    //     let runtime = match tokio::runtime::Builder::new_multi_thread()
    //         .enable_io()
    //         .enable_time()
    //         .worker_threads(1)
    //         .build()
    //     {
    //         Ok(rt) => rt,
    //         Err(e) => {
    //             log::error!(
    //                 "{}",
    //                 serde_json::json!({
    //                     "error": e.to_string(),
    //                 })
    //                 .to_string()
    //             );
    //             panic!()
    //         }
    //     };
    //     runtime.block_on(async {
    //         controller.await;
    //         tokio::time::sleep(std::time::Duration::MAX).await;
    //     });
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
