pub mod bbo;
pub mod trade;

pub fn run(
    sender: crossbeam_channel::Sender<crate::message::Message>,
    instruments: &std::collections::HashSet<crate::structs::instrument::Instrument>,
) -> Vec<futures::future::BoxFuture<'static, ()>> {
    let mut rt = Vec::<futures::future::BoxFuture<'static, ()>>::new();

    let sender_clone = sender.clone();
    let insts_clone = instruments.iter().copied().collect();
    let awork = crate::util::websocket::awork(
        crate::env_var::MARKET_FTX_WSS.to_string(),
        bbo::wss(sender_clone),
        bbo::State { insts: insts_clone },
    );
    rt.push(Box::pin(awork));

    let insts_clone = instruments.iter().copied().collect();
    let awork = crate::util::websocket::awork(
        crate::env_var::MARKET_FTX_WSS.to_string(),
        trade::wss(sender),
        trade::State { insts: insts_clone },
    );
    rt.push(Box::pin(awork));

    rt
}
