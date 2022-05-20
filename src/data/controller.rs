#[derive(Default)]
pub struct State {
    handles: std::collections::HashMap<
        proper_market_api::Exchange,
        (
            std::collections::HashSet<crate::structs::instrument::Instrument>,
            Vec<tokio::task::JoinHandle<()>>,
        ),
    >,
}

macro_rules! handle_exchange_state {
    ($state:expr, $insts:expr, $sender:expr, $enum:expr, $runner:expr) => {
        let exch_state = $state
            .handles
            .entry($enum)
            .or_insert((std::collections::HashSet::new(), vec![]));
        let exch_insts = $insts
            .iter()
            .filter(|i| i.exchange == $enum)
            .map(|s| *s)
            .collect();
        if exch_state.0 != exch_insts {
            for handle in exch_state.1.iter() {
                handle.abort();
            }
            let mut handles = vec![];
            let tokio_tasks = $runner($sender.clone(), &exch_insts);
            for tokio_task in tokio_tasks.into_iter() {
                handles.push(tokio::spawn(tokio_task));
            }
            exch_state.0 = exch_insts.into_iter().collect();
            exch_state.1 = handles;
        }
    };
}

pub async fn work(
    sender: crossbeam_channel::Sender<crate::message::Message>,
    state: std::sync::Arc<tokio::sync::Mutex<State>>,
    insts: Vec<crate::structs::instrument::Instrument>,
) {
    let mut state = state.lock().await;

    handle_exchange_state!(
        state,
        insts,
        sender,
        proper_market_api::Exchange::binance,
        crate::data::exchange::binance::r#async::run
    );

    handle_exchange_state!(
        state,
        insts,
        sender,
        proper_market_api::Exchange::ftx,
        crate::data::exchange::ftx::r#async::run
    );

    // crate::ftx::run(sender.clone(), insts.iter().filter(|i| i.exchange == proper_market_api::Exchange::ftx).map(|s| *s).collect());
}
