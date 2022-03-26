#[derive(Default)]
pub struct State {
    handles: std::collections::HashMap<
        proper_market_api::Exchange,
        (
            std::collections::HashSet<crate::structs::symbol::Symbol>,
            Vec<tokio::task::JoinHandle<()>>,
        ),
    >,
}

pub async fn work(
    sender: crossbeam_channel::Sender<crate::message::Message>,
    state: std::sync::Arc<tokio::sync::Mutex<State>>,
    insts: Vec<crate::structs::symbol::Symbol>,
) {
    let mut state = state.lock().await;

    let binance_state = state
        .handles
        .entry(proper_market_api::Exchange::binance)
        .or_insert((std::collections::HashSet::new(), vec![]));
    let binance_insts = insts
        .iter()
        .filter(|i| i.exchange == proper_market_api::Exchange::binance)
        .map(|s| *s)
        .collect();
    if binance_state.0 != binance_insts {
        for handle in binance_state.1.iter() {
            handle.abort();
        }
        let mut handles = vec![];
        let tokio_tasks =
            crate::data::exchange::binance::r#async::run(sender.clone(), &binance_insts);
        for tokio_task in tokio_tasks.into_iter() {
            handles.push(tokio::spawn(tokio_task));
        }
        binance_state.0 = binance_insts.into_iter().collect();
        binance_state.1 = handles;
    }

    let ftx_state = state
        .handles
        .entry(proper_market_api::Exchange::ftx)
        .or_insert((std::collections::HashSet::new(), vec![]));
    let ftx_insts = insts
        .iter()
        .filter(|i| i.exchange == proper_market_api::Exchange::ftx)
        .map(|s| *s)
        .collect();
    if ftx_state.0 != ftx_insts {
        for handle in ftx_state.1.iter() {
            handle.abort();
        }
        let mut handles = vec![];
        let tokio_tasks = crate::data::exchange::ftx::r#async::run(sender.clone(), &ftx_insts);
        for tokio_task in tokio_tasks.into_iter() {
            handles.push(tokio::spawn(tokio_task));
        }
        ftx_state.0 = ftx_insts.into_iter().collect();
        ftx_state.1 = handles;
    }

    // crate::ftx::run(sender.clone(), insts.iter().filter(|i| i.exchange == proper_market_api::Exchange::ftx).map(|s| *s).collect());
}
