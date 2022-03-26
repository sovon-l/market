use std::sync::{Arc, Mutex};

pub fn publisher(
    ctx: Arc<Mutex<zmq::Context>>,
    rcv: crossbeam_channel::Receiver<crate::message::Message>,
    connect: impl Fn(&zmq::Socket),
) {
    let socket = {
        let ctx = ctx.lock().unwrap();
        ctx.socket(zmq::PUB).unwrap()
    };

    connect(&socket);

    loop {
        match rcv.recv() {
            Ok(msg) => {
                // let market_timestamp = match msg {
                //     crate::message::Message::BboMsg(ref b) => b.market_timestamp,
                //     crate::message::Message::TradesMsg(ref t) => t.market_timestamp,
                // };
                socket.send(&msg, zmq::DONTWAIT).unwrap();
                // log::info!(
                //     "{}",
                //     simd_json::to_string(&simd_json::json!({
                //         "ts": market_timestamp,
                //         "process_time": (chrono::Utc::now().timestamp_nanos() - market_timestamp as i64) as f64 / 1_000_000.0,
                //     })).unwrap(),
                // );
            }
            Err(e) => log::error!("{}", e),
        }
    }
}
