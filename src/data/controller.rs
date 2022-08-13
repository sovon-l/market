#[derive(Default)]
pub struct State {
    handles: std::collections::HashMap<
        proper_ma_structs::structs::market::exchange::Exchange,
        (
            std::collections::HashSet<proper_ma_structs::structs::market::instrument::Instrument>,
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
    sender: impl messenger::traits::ChannelSender<crate::message::Message> + Clone + Send + 'static,
    state: std::sync::Arc<tokio::sync::Mutex<State>>,
    insts: Vec<proper_ma_structs::structs::market::instrument::Instrument>,
) {
    let mut state = state.lock().await;

    handle_exchange_state!(
        state,
        insts,
        sender,
        proper_ma_structs::structs::market::exchange::Exchange::Binance,
        crate::data::exchange::binance::r#async::run
    );

    handle_exchange_state!(
        state,
        insts,
        sender,
        proper_ma_structs::structs::market::exchange::Exchange::Ftx,
        crate::data::exchange::ftx::r#async::run
    );

    // crate::ftx::run(sender.clone(), insts.iter().filter(|i| i.exchange == proper_ma_structs::structs::market::exchange::Exchange::ftx).map(|s| *s).collect());
}
