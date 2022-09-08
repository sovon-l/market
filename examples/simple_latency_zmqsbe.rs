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

fn main() {
    //     simple_logger::init_with_level(log::Level::Debug).unwrap();

    //     let m = std::sync::Arc::new(std::sync::Mutex::new(zmq::Context::new()));

    //     let ctx = m.lock().unwrap();

    //     let subscriber = ctx.socket(zmq::SUB).unwrap();
    //     subscriber.connect("tcp://127.0.0.1:8000").unwrap();

    //     messenger::subscriber::subscriber_loop(
    //         subscriber,
    //         market::api::market_listener::MarketListener {
    //             inner: LatencyListener {},
    //         },
    //     );
}
