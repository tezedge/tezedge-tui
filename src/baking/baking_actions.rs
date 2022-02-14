use redux_rs::EnablingCondition;

use crate::automaton::State;

use super::{BakingRightsPerLevel, BlockApplicationStatistics, PerPeerBlockStatisticsVector};

// ApplicationStatistics

#[derive(Debug, Clone)]
pub struct ApplicationStatisticsGetAction {
    pub level: i32,
}

impl EnablingCondition<State> for ApplicationStatisticsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationStatisticsReceivedAction {
    pub application_statistics: Vec<BlockApplicationStatistics>,
}

impl EnablingCondition<State> for ApplicationStatisticsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct PerPeerBlockStatisticsGetAction {
    pub level: i32,
}

impl EnablingCondition<State> for PerPeerBlockStatisticsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct PerPeerBlockStatisticsReceivedAction {
    pub per_peer_block_statistics: PerPeerBlockStatisticsVector,
}

impl EnablingCondition<State> for PerPeerBlockStatisticsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct BakingRightsGetAction {}

impl EnablingCondition<State> for BakingRightsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
#[derive(Debug, Clone)]
pub struct BakingRightsReceivedAction {
    pub rights: Vec<BakingRightsPerLevel>,
}

impl EnablingCondition<State> for BakingRightsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
