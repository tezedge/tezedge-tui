use crate::automaton::{Action, ActionWithMeta, State};

use super::{PerPeerBlockStatisticsExtended, PerPeerBlockStatisticsExtendedVector};

pub fn baking_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::ApplicationStatisticsReceived(action) => {
            for stats in &action.application_statistics {
                state
                    .baking
                    .application_statistics
                    .insert(stats.block_hash.clone(), stats.clone());
            }
        }
        Action::PerPeerBlockStatisticsReceived(action) => {
            let extended: PerPeerBlockStatisticsExtendedVector = action
                .per_peer_block_statistics
                .clone()
                .into_iter()
                .map(PerPeerBlockStatisticsExtended::from)
                .collect();
            if let Some(peer_stats) = action.per_peer_block_statistics.get(0) {
                state
                    .baking
                    .per_peer_block_statistics
                    .insert(peer_stats.block_hash.clone(), extended.clone());
            }

            state.baking.baking_table.content = extended;

            state.baking.baking_table.sort_content(state.delta_toggle);
        }
        Action::CurrentHeadHeaderChanged(_) => {
            // state.baking.baking_table.content.clear();
        }
        Action::BakingRightsReceived(action) => {
            state.baking.baking_rights.add(&action.rights);
        }
        _ => {}
    }
}
