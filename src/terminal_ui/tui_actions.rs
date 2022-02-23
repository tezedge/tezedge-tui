use crossterm::event::KeyModifiers;
use redux_rs::EnablingCondition;
use serde::{Deserialize, Serialize};

use crate::{
    automaton::State,
    services::rpc_service_async::{CurrentHeadHeader, CurrentHeadMetadata, NetworkConstants},
};

use super::ActivePage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawScreenAction {}

impl EnablingCondition<State> for DrawScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawScreenSuccessAction {
    pub screen_width: u16,
}

impl EnablingCondition<State> for DrawScreenSuccessAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawScreenFailiureAction {
    // TODO: return proper error variants
    _error: String,
}

impl EnablingCondition<State> for DrawScreenFailiureAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeScreenAction {
    pub screen: ActivePage,
}

impl EnablingCondition<State> for ChangeScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiRightKeyPushedAction {}

impl EnablingCondition<State> for TuiRightKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiLeftKeyPushedAction {}

impl EnablingCondition<State> for TuiLeftKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiUpKeyPushedAction {}

impl EnablingCondition<State> for TuiUpKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiDownKeyPushedAction {}

impl EnablingCondition<State> for TuiDownKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiSortKeyPushedAction {
    pub modifier: KeyModifiers,
}

impl EnablingCondition<State> for TuiSortKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiDeltaToggleKeyPushedAction {}

impl EnablingCondition<State> for TuiDeltaToggleKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiWidgetSelectionKeyPushedAction {}

impl EnablingCondition<State> for TuiWidgetSelectionKeyPushedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentHeadHeaderGetAction {}

impl EnablingCondition<State> for CurrentHeadHeaderGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentHeadHeaderRecievedAction {
    pub current_head_header: CurrentHeadHeader,
}

impl EnablingCondition<State> for CurrentHeadHeaderRecievedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentHeadHeaderChangedAction {
    pub current_head_header: CurrentHeadHeader,
}

impl EnablingCondition<State> for CurrentHeadHeaderChangedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleChangedAction {
    pub new_cycle: i32,
    pub at_level: i32,
}

impl EnablingCondition<State> for CycleChangedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConstantsGetAction {}

impl EnablingCondition<State> for NetworkConstantsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConstantsReceivedAction {
    pub constants: NetworkConstants,
}

impl EnablingCondition<State> for NetworkConstantsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentHeadMetadataGetAction {}

impl EnablingCondition<State> for CurrentHeadMetadataGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentHeadMetadataReceivedAction {
    pub metadata: CurrentHeadMetadata,
}

impl EnablingCondition<State> for CurrentHeadMetadataReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentHeadMetadataChangedAction {
    pub new_metadata: CurrentHeadMetadata,
}

impl EnablingCondition<State> for CurrentHeadMetadataChangedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestRemoteLevelGetAction {}

impl EnablingCondition<State> for BestRemoteLevelGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestRemoteLevelReceivedAction {
    pub level: Option<i32>,
}

impl EnablingCondition<State> for BestRemoteLevelReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestRemoteLevelChangedAction {
    pub level: Option<i32>,
}

impl EnablingCondition<State> for BestRemoteLevelChangedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
