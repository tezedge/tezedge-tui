// This module is temporary and will be removed

use slog::info;

use crate::automaton::{Action, ActionWithMeta, State};

pub fn action_logger_reducer(state: &mut State, action: &ActionWithMeta) {
    // log all actions
    // let deserialized_action = serde_json::to_string(action).unwrap_or_default();
    state.recorded_actions.push(action.clone());

    // info!(state.log, "{}", deserialized_action);
}