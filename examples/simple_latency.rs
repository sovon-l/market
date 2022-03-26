use std::sync::{Arc, Mutex};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let m = Arc::new(Mutex::new(zmq::Context::new()));

    let ctx = m.lock().unwrap();

    let subscriber = ctx.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://127.0.0.1:8000").unwrap();
    subscriber.set_subscribe(b"").unwrap();

    while let Ok(msg) = subscriber.recv_bytes(0) {
        let time_now = chrono::Utc::now().timestamp_nanos();
        let (time_msg, symbol) = match market::message::decode_message(&msg) {
            market::message::Message::BboMsg(b) => (b.market_timestamp, b.symbol),
            market::message::Message::TradesMsg(t) => (t.market_timestamp, t.symbol),
        };
        log::info!(
            "{}",
            serde_json::json!({
                "ts": time_msg,
                "latency_ms": (time_now - time_msg as i64) as f64 / 1_000_000.0,
                "symbol": symbol.to_string(),
            })
        );
    }
}
