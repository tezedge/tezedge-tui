use redux_rs::EnablingCondition;

use crate::automaton::State;

use super::ActivePage;

#[derive(Debug, Clone)]
pub struct DrawScreenAction {}

impl EnablingCondition<State> for DrawScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ChangeScreenAction {
    pub screen: ActivePage,
}

impl EnablingCondition<State> for ChangeScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
