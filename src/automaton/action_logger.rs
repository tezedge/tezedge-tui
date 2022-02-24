// This module is temporary and will be removed

use slog::info;

use crate::automaton::{Action, ActionWithMeta, State};

pub fn action_logger_reducer(state: &mut State, action: &ActionWithMeta) {
    // capture only top level actions
    if action.depth == 0 {
        state.recorded_actions.push(action.clone());
    }
}
