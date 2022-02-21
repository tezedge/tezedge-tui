use crossterm::event::KeyModifiers;
use redux_rs::EnablingCondition;

use crate::{
    automaton::State,
    services::rpc_service::{CurrentHeadHeader, NetworkConstants},
};

use super::ActivePage;

#[derive(Debug, Clone)]
pub struct DrawScreenAction {}

impl EnablingCondition<State> for DrawScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct DrawScreenSuccessAction {
    pub screen_width: u16,
}

impl EnablingCondition<State> for DrawScreenSuccessAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct DrawScreenFailiureAction {
    // TODO: return proper error variants
    _error: String,
}

impl EnablingCondition<State> for DrawScreenFailiureAction {
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

#[derive(Debug, Clone)]
pub struct TuiRightKeyPushedAction {}

impl EnablingCondition<State> for TuiRightKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct TuiLeftKeyPushedAction {}

impl EnablingCondition<State> for TuiLeftKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct TuiUpKeyPushedAction {}

impl EnablingCondition<State> for TuiUpKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct TuiDownKeyPushedAction {}

impl EnablingCondition<State> for TuiDownKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct TuiSortKeyPushedAction {
    pub modifier: KeyModifiers,
}

impl EnablingCondition<State> for TuiSortKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct TuiDeltaToggleKeyPushedAction {}

impl EnablingCondition<State> for TuiDeltaToggleKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct TuiWidgetSelectionKeyPushedAction {}

impl EnablingCondition<State> for TuiWidgetSelectionKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct CurrentHeadHeaderGetAction {}

impl EnablingCondition<State> for CurrentHeadHeaderGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct CurrentHeadHeaderRecievedAction {
    pub current_head_header: CurrentHeadHeader,
}

impl EnablingCondition<State> for CurrentHeadHeaderRecievedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct CurrentHeadHeaderChangedAction {
    pub current_head_header: CurrentHeadHeader,
}

impl EnablingCondition<State> for CurrentHeadHeaderChangedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct CycleChangedAction {
    pub new_cycle: i32,
}

impl EnablingCondition<State> for CycleChangedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct NetworkConstantsGetAction {}

impl EnablingCondition<State> for NetworkConstantsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct NetworkConstantsReceivedAction {
    pub constants: NetworkConstants,
}

impl EnablingCondition<State> for NetworkConstantsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
