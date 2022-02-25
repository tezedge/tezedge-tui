use redux_rs::EnablingCondition;
use serde::{Deserialize, Serialize};

use crate::automaton::State;

use super::OperationsStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationsStatisticsGetAction {}

impl EnablingCondition<State> for OperationsStatisticsGetAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationsStatisticsReceivedAction {
    pub operations_statistics: OperationsStats,
}

impl EnablingCondition<State> for OperationsStatisticsReceivedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
