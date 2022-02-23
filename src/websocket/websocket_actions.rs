use redux_rs::EnablingCondition;
use serde::{Serialize, Deserialize};

use crate::{automaton::State, services::ws_service::WebsocketMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketReadAction {}

impl EnablingCondition<State> for WebsocketReadAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketMessageReceivedAction {
    pub websocket_message: Vec<WebsocketMessage>,
}

impl EnablingCondition<State> for WebsocketMessageReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
