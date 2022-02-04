use crate::automaton::{Action, ActionWithMeta, State};

pub fn baking_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::ApplicationStatisticsReceived(action) => {
            state.baking.application_statistics = action.application_statistics.clone();
        }
        Action::PerPeerBlockStatisticsReceivedAction(action) => {
            state.baking.per_peer_block_statistics = action.per_peer_block_statistics.clone();
        }
        _ => {}
    }
}
