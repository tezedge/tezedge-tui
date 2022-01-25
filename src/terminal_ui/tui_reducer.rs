use crate::automaton::{Action, ActionWithMeta, State};

pub fn tui_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::ChangeScreen(action) => {
            state.ui.active_page = action.screen.clone();
        }
        Action::Init(_) => {}
        _ => {}
    }
}
