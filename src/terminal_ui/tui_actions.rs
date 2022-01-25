use redux_rs::EnablingCondition;

use crate::automaton::State;

#[derive(Debug, Clone)]
pub struct DrawScreenAction {}

impl EnablingCondition<State> for DrawScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
