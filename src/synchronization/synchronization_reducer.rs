use crate::{
    automaton::{Action, ActionWithMeta, State},
    services::ws_service::WebsocketMessage,
};

pub fn synchronization_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::Init(_) => {}
        Action::WebsocketMessageReceived(action) => {
            for message in &action.websocket_message {
                match &message {
                    WebsocketMessage::IncomingTransfer(incoming_transfer) => {
                        state.synchronization.incoming_transfer = incoming_transfer.clone();
                    }
                    WebsocketMessage::BlockStatus(block_status) => {
                        state.synchronization.block_metrics = block_status.clone();
                    }
                    WebsocketMessage::BlockApplicationStatus(block_application_status) => {
                        state.synchronization.aplication_status = block_application_status.clone();
                    }
                    WebsocketMessage::ChainStatus(chain_status) => {
                        state.synchronization.cycle_data = chain_status.chain.clone();
                    }
                    WebsocketMessage::PeersMetrics(peer_metrics) => {
                        state.synchronization.peer_metrics = peer_metrics.clone();
                    }
                }
            }
        }
        _ => {}
    }
}
