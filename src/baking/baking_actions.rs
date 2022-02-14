use redux_rs::EnablingCondition;

use crate::automaton::State;

use super::{BlockApplicationStatistics, PerPeerBlockStatisticsVector, BakingRightsPerLevel};

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

#[derive(Debug, Clone)]
pub struct BlockBakedAction {
    pub level: i32,
}

impl EnablingCondition<State> for BlockBakedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationStatisticsBakedGetAction {
    pub level: i32,
}

impl EnablingCondition<State> for ApplicationStatisticsBakedGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationStatisticsBakedReceivedAction {
    pub application_statistics: Vec<BlockApplicationStatistics>,
}

impl EnablingCondition<State> for ApplicationStatisticsBakedReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct PerPeerBlockStatisticsBakedGetAction {
    pub level: i32,
}

impl EnablingCondition<State> for PerPeerBlockStatisticsBakedGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct PerPeerBlockStatisticsBakedReceivedAction {
    pub per_peer_block_statistics: PerPeerBlockStatisticsVector,
}

impl EnablingCondition<State> for PerPeerBlockStatisticsBakedReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}