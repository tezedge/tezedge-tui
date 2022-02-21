use std::collections::BTreeMap;

use crate::automaton::{Action, ActionWithMeta, State};

use super::{
    EndorsementOperationSummary, EndorsementRightsWithTime, EndorsementState, EndorsementStatus,
    EndorsementStatusSortable, EndorsementStatusSortableVec,
};

pub fn endorsementrs_reducer(state: &mut State, action: &ActionWithMeta) {
    // let action_time = action.time_as_nanos();

    match &action.action {
        Action::EndorsementsRightsReceived(action) => {
            state.endorsmenents.endorsement_rights = action.endorsement_rights.clone();
        }
        Action::EndorsementsStatusesReceived(action) => {
            let slot_mapped: BTreeMap<u32, EndorsementStatus> = action
                .endorsements_statuses
                .clone()
                .into_iter()
                .map(|(_, v)| (v.slot, v))
                .collect();

            if !slot_mapped.is_empty() {
                let mut sumary: BTreeMap<EndorsementState, usize> = BTreeMap::new();

                let endorsement_operation_time_statistics: EndorsementStatusSortableVec = state
                    .endorsmenents
                    .endorsement_rights
                    .iter()
                    .map(|(k, v)| {
                        if let Some((_, status)) =
                            slot_mapped.iter().find(|(slot, _)| v.contains(slot))
                        {
                            let status = status.to_sortable(k.to_string(), v.len());
                            let state_count = sumary.entry(status.state.clone()).or_insert(0);
                            *state_count += status.slot_count;
                            status
                        } else {
                            let status = EndorsementStatusSortable::new(k.to_string(), v.len());
                            let state_count = sumary.entry(EndorsementState::Missing).or_insert(0);
                            *state_count += status.slot_count;
                            status
                        }
                    })
                    .collect();

                let delta_toggle = state.delta_toggle;

                state.endorsmenents.endoresement_status_summary = sumary;
                state.endorsmenents.endorsement_table.content =
                    endorsement_operation_time_statistics;

                state
                    .endorsmenents
                    .endorsement_table
                    .sort_content(delta_toggle);
            }
        }
        Action::EndorsementsRightsWithTimeReceived(action) => {
            state.endorsmenents.endorsement_rights_with_time =
                EndorsementRightsWithTime::new(&action.rights);
        }
        Action::MempoolEndorsementStatsReceived(stats) => {
            // let injected_endorsement_stats = stats.stats.iter().find(|(oph, stats)| stats.is_injected());

            if let Some((_, injected_endrosement_stats)) =
                stats.stats.iter().find(|(_, stats)| stats.is_injected())
            {
                let current_level = state.current_head_header.level;
                // TODO: simple insert would suffice, rigth?
                let injected_stat = state
                    .endorsmenents
                    .injected_endorsement_stats
                    .entry(current_level)
                    .or_default();
                *injected_stat = injected_endrosement_stats.clone();
            }
        }
        Action::CurrentHeadHeaderChanged(action) => {
            // we update the last summary AFTER the endorsement happened, so we use the notion of the previous head to get the stored data
            if let Some((endorsing_level, _)) = state
                .endorsmenents
                .endorsement_rights_with_time
                .next_endorsing(
                    state.previous_head_header.level,
                    state.previous_head_header.timestamp,
                    state.network_constants.minimal_block_delay,
                )
            {
                if endorsing_level == state.previous_head_header.level {
                    state.endorsmenents.last_endrosement_operation_level = endorsing_level;

                    let block_stats = state.baking.application_statistics.get(&state.previous_head_header.hash).cloned();

                    let op_stats = state
                        .endorsmenents
                        .injected_endorsement_stats
                        .get(&state.previous_head_header.level)
                        .cloned()
                        .unwrap_or_default();
                    let injected_endorsement_summary = EndorsementOperationSummary::new(
                        state.previous_head_header.timestamp,
                        op_stats,
                        block_stats
                    );

                    state.endorsmenents.last_injected_endorsement_summary =
                        injected_endorsement_summary;
                }
            }
        }
        _ => {}
    }
}
