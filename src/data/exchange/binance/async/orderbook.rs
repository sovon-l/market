

#[derive(Default)]
pub struct State {
    last_ts: Option<(u64, u64)>,
}

impl crate::util::websocket::WssState for State {
    fn init_messages(&self) -> Vec<tokio_tungstenite::tungstenite::Message> {
        Vec::new()
    }
}

pub fn wss(
    mut sender: impl messenger::traits::ChannelSender<crate::message::Message>,
) -> impl FnMut(
    std::time::SystemTime,
    &mut State,
    crate::util::websocket::Message<()>,
) -> Option<tokio_tungstenite::tungstenite::Message> {
    move |time_recv, state, msg| {
        None
    }
}