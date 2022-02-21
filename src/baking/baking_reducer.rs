use crate::automaton::{Action, ActionWithMeta, State};

use super::{BakingRights, BakingSummary, BlockApplicationSummary};

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
            if let Some(peer_stats) = action.per_peer_block_statistics.get(0) {
                state.baking.per_peer_block_statistics.insert(
                    peer_stats.block_hash.clone(),
                    action.per_peer_block_statistics.clone(),
                );
            }

            // TODO: use only one of these fields...
            state.baking.baking_table.content = action.per_peer_block_statistics.clone();

            state.baking.baking_table.sort_content(state.delta_toggle);
        }
        Action::CurrentHeadHeaderChanged(action) => {
            state.baking.baking_table.content.clear();

            if let Some((baking_level, _)) = state
                .baking
                .baking_rights
                .next_baking(action.current_head_header.level)
            {
                if baking_level == action.current_head_header.level {
                    state.baking.last_baked_block_level = Some(action.current_head_header.level);
                    state.baking.last_baked_block_hash =
                        Some(action.current_head_header.hash.clone());

                    let block_application_summary = if let Some(application_statistics) = state
                        .baking
                        .application_statistics
                        .get(&state.current_head_header.hash)
                    {
                        BlockApplicationSummary::from(application_statistics.clone())
                    } else {
                        BlockApplicationSummary::default()
                    };

                    let per_peer = if let Some(last_baked_per_peer_stats) = state
                        .baking
                        .per_peer_block_statistics
                        .get(&state.current_head_header.hash)
                    {
                        last_baked_per_peer_stats.clone()
                    } else {
                        Vec::new()
                    };

                    let summary = BakingSummary::new(baking_level, state.previous_head_header.clone(), block_application_summary, per_peer);

                    state.baking.last_baking_summary = summary;
                }
            }
        }
        Action::BakingRightsReceived(action) => {
            state.baking.baking_rights = BakingRights::new(&action.rights);
        }
        _ => {}
    }
}
