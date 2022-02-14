use crate::automaton::{Action, ActionWithMeta, State};

use super::{BakingRights, BakingSummary, BlockApplicationSummary};

pub fn baking_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::ApplicationStatisticsReceived(action) => {
            state.baking.application_statistics = action.application_statistics.clone();
        }
        Action::PerPeerBlockStatisticsReceived(action) => {
            state.baking.per_peer_block_statistics = action.per_peer_block_statistics.clone();

            // TODO: use only one of these fields...
            state.baking.baking_table.content = action.per_peer_block_statistics.clone();

            state.baking.baking_table.sort_content(state.delta_toggle);
        }
        Action::ApplicationStatisticsBakedReceived(action) => {
            state.baking.last_baked_application_statistics = action.application_statistics.clone();
        }
        Action::PerPeerBlockStatisticsBakedReceivedAction(action) => {
            state.baking.last_baked_per_peer_block_statistics = action.per_peer_block_statistics.clone();
        }
        Action::CurrentHeadHeaderChanged(action) => {
            state.baking.baking_table.content.clear();

            if let Some((baking_level, _)) = state.baking.baking_rights.next_baking(action.current_head_header.level) {
                if baking_level == action.current_head_header.level {
                    state.baking.last_baked_block_level = Some(action.current_head_header.level);
                }
            }

        },
        Action::BakingRightsReceived(action) => {
            state.baking.baking_rights = BakingRights::new(&action.rights);
        }
        _ => {}
    }
}
