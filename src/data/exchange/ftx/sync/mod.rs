pub mod bbo;
pub mod trade;

pub fn run(
    sender: crossbeam_channel::Sender<crate::message::Message>,
    instruments: Vec<crate::structs::symbol::Symbol>,
) {
    let sender_clone = sender.clone();
    let insts_clone = instruments.clone();
    std::thread::Builder::new()
        .name("ftx bbo".into())
        .spawn(move || {
            crate::util::websocket::work(
                &*crate::env_var::MARKET_FTX_WSS,
                bbo::wss(sender_clone),
                bbo::State { insts: insts_clone },
            )
        })
        .unwrap();

    std::thread::Builder::new()
        .name("ftx trades".into())
        .spawn(move || {
            crate::util::websocket::work(
                &*crate::env_var::MARKET_FTX_WSS,
                trade::wss(sender),
                trade::State { insts: instruments },
            )
        })
        .unwrap();
}
