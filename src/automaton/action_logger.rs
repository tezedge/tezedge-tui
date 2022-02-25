// This module is temporary and will be removed

use crate::automaton::{ActionWithMeta, State};

pub fn action_logger_reducer(state: &mut State, action: &ActionWithMeta) {
    // capture only top level actions
    if state.record_actions && action.depth == 0 {
        state.recorded_actions.push(action.clone());
    }
}
