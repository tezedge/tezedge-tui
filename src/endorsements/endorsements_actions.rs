use redux_rs::EnablingCondition;

use crate::{
    automaton::State,
    services::rpc_service::{CurrentHeadHeader, EndorsementRights, EndorsementStatuses},
};

#[derive(Debug, Clone)]
pub struct EndorsementsRightsGetAction {
    pub block: String,
    pub level: i32,
}

impl EnablingCondition<State> for EndorsementsRightsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct EndorsementsRightsReceivedAction {
    pub endorsement_rights: EndorsementRights,
}

impl EnablingCondition<State> for EndorsementsRightsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct EndorsementsStatusesReceivedAction {
    pub endorsements_statuses: EndorsementStatuses,
}

impl EnablingCondition<State> for EndorsementsStatusesReceivedAction {
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
pub struct DrawEndorsementsScreenAction {
    pub current_head_header: CurrentHeadHeader,
}

impl EnablingCondition<State> for DrawEndorsementsScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
