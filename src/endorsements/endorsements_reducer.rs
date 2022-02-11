use std::collections::BTreeMap;

use crate::automaton::{Action, ActionWithMeta, State};

use super::{
    EndorsementState, EndorsementStatus, EndorsementStatusSortable, EndorsementStatusSortableVec,
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
        _ => {}
    }
}
