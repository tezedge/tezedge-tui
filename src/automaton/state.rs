use serde::{Deserialize, Serialize};
use slog::Logger;

use crate::{
    baking::BakingState,
    endorsements::EndrosementsState,
    operations::OperationsStatisticsState,
    services::rpc_service_async::{CurrentHeadHeader, CurrentHeadMetadata, NetworkConstants},
    synchronization::SynchronizationState,
    terminal_ui::UiState,
};

use super::ActionWithMeta;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub network_constants: NetworkConstants,
    pub last_applied_level: i32,
    pub current_head_header: CurrentHeadHeader,
    pub current_head_metadata: CurrentHeadMetadata,
    pub previous_head_header: CurrentHeadHeader,
    pub best_remote_level: Option<i32>,
    pub baker_address: Option<String>,

    pub synchronization: SynchronizationState,
    pub endorsmenents: EndrosementsState,
    pub operations_statistics: OperationsStatisticsState,
    pub baking: BakingState,

    pub delta_toggle: bool,

    pub ui: UiState,

    #[serde(skip)]
    pub log: crate::automaton::Logger,
    #[serde(skip)]
    pub recorded_actions: Vec<ActionWithMeta>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.network_constants == other.network_constants
            && self.last_applied_level == other.last_applied_level
            && self.current_head_header == other.current_head_header
            && self.current_head_metadata == other.current_head_metadata
            && self.previous_head_header == other.previous_head_header
            && self.best_remote_level == other.best_remote_level
            && self.baker_address == other.baker_address
            && self.synchronization == other.synchronization
            && self.endorsmenents == other.endorsmenents
            && self.operations_statistics == other.operations_statistics
            && self.baking == other.baking
            && self.delta_toggle == other.delta_toggle
            && self.ui == other.ui
    }
}

impl State {
    pub fn new(baker_address: Option<String>, log: Logger) -> Self {
        Self {
            log: crate::automaton::Logger(log),
            baker_address,
            delta_toggle: true,
            current_head_header: Default::default(),
            current_head_metadata: Default::default(),
            previous_head_header: Default::default(),
            last_applied_level: Default::default(),
            synchronization: Default::default(),
            endorsmenents: Default::default(),
            operations_statistics: Default::default(),
            baking: Default::default(),
            ui: Default::default(),
            network_constants: Default::default(),
            best_remote_level: Default::default(),
            recorded_actions: Vec::new(),
        }
    }
}
