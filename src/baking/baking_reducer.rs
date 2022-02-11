use crate::automaton::{Action, ActionWithMeta, State};

pub fn baking_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::ApplicationStatisticsReceived(action) => {
            state.baking.application_statistics = action.application_statistics.clone();
        }
        Action::PerPeerBlockStatisticsReceivedAction(action) => {
            state.baking.per_peer_block_statistics = action.per_peer_block_statistics.clone();

            // TODO: use only one of these fields...
            state.baking.baking_table.content = action.per_peer_block_statistics.clone();

            state.baking.baking_table.sort_content(state.delta_toggle);
        }
        _ => {}
    }
}
