use crate::{
    automaton::{Action, ActionWithMeta, State},
    extensions::SortableByFocus,
};

use super::OperationsStatsSortable;

pub fn operations_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::Init(_) => {}
        Action::OperationsStatisticsReceived(action) => {
            let sort_focus = state
                .operations_statistics
                .main_operation_statistics_table
                .selected();
            let delta_toggle = state.delta_toggle;

            let mut sortable: OperationsStatsSortable = action
                .operations_statistics
                .clone()
                .into_iter()
                .map(|(k, v)| v.to_statistics_sortable(k))
                .collect();

            sortable.sort_by_focus(sort_focus, delta_toggle);

            state.operations_statistics.operations_statistics =
                action.operations_statistics.clone();
            state.operations_statistics.operations_statistics_sortable = sortable;
        }
        _ => {}
    }
}
