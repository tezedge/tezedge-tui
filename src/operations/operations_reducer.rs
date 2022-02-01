use crate::automaton::{Action, ActionWithMeta, State};

use super::OperationsStatsSortable;

pub fn operations_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::Init(_) => {}
        Action::OperationsStatisticsReceived(action) => {
            let delta_toggle = state.delta_toggle;

            let sortable: OperationsStatsSortable = action
                .operations_statistics
                .clone()
                .into_iter()
                .map(|(k, v)| v.to_statistics_sortable(k))
                .collect();

            state.operations_statistics.operations_statistics =
                action.operations_statistics.clone();
            state
                .operations_statistics
                .main_operation_statistics_table
                .content = sortable;
            state
                .operations_statistics
                .main_operation_statistics_table
                .sort_content(delta_toggle);
        }
        _ => {}
    }
}
