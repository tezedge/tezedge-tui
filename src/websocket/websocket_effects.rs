use crate::{
    automaton::{Action, ActionWithMeta, Store},
    services::{ws_service::WebsocketService, Service},
};

use super::WebsocketMessageReceivedAction;

pub fn websocket_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::Init(_) => {}
        Action::WebsocketRead(_) => {
            while let Ok(websocket_message) = store.service().ws().message_try_recv() {
                store.dispatch(WebsocketMessageReceivedAction { websocket_message });
            }
        }
        _ => {}
    }
}
