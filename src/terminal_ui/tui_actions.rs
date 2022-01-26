use crossterm::event::KeyModifiers;
use redux_rs::EnablingCondition;
use tui::terminal::CompletedFrame;

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
    error: String,
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