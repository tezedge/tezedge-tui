use serde::{Serialize, Deserialize};
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
    pub recorded_actions: Vec<ActionWithMeta>
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
