use redux_rs::EnablingCondition;
use serde::{Deserialize, Serialize};

use crate::{automaton::State, services::rpc_service_async::CurrentHeadHeader};

use super::{
    EndorsementRights, EndorsementRightsWithTimePerLevel, EndorsementStatuses,
    MempoolEndorsementStats,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorsementsRightsGetAction {
    pub block: String,
    pub level: i32,
}

impl EnablingCondition<State> for EndorsementsRightsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorsementsRightsReceivedAction {
    pub endorsement_rights: EndorsementRights,
}

impl EnablingCondition<State> for EndorsementsRightsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorsementsStatusesGetAction {}

impl EnablingCondition<State> for EndorsementsStatusesGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorsementsStatusesReceivedAction {
    pub endorsements_statuses: EndorsementStatuses,
}

impl EnablingCondition<State> for EndorsementsStatusesReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawEndorsementsScreenAction {
    pub current_head_header: CurrentHeadHeader,
}

impl EnablingCondition<State> for DrawEndorsementsScreenAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorsementsRightsWithTimeGetAction {}

impl EnablingCondition<State> for EndorsementsRightsWithTimeGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorsementsRightsWithTimeReceivedAction {
    pub rights: Vec<EndorsementRightsWithTimePerLevel>,
}

impl EnablingCondition<State> for EndorsementsRightsWithTimeReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

// MempoolEndorsementStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolEndorsementStatsGetAction {}

impl EnablingCondition<State> for MempoolEndorsementStatsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolEndorsementStatsReceivedAction {
    pub stats: MempoolEndorsementStats,
}

impl EnablingCondition<State> for MempoolEndorsementStatsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
