pub mod bbo;
pub mod trade;

pub fn run(
    sender: crossbeam_channel::Sender<crate::message::Message>,
    instruments: Vec<crate::structs::instrument::Instrument>,
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

    let sender_clone = sender.clone();
    let insts_clone = instruments.clone();
    std::thread::Builder::new()
        .name("ftx trades".into())
        .spawn(move || {
            crate::util::websocket::work(
                &*crate::env_var::MARKET_FTX_WSS,
                trade::wss(sender_clone),
                trade::State { insts: insts_clone },
            )
        })
        .unwrap();
}
