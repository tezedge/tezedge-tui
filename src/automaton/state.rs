use slog::Logger;

use crate::{
    baking::BakingState,
    endorsements::EndrosementsState,
    operations::OperationsStatisticsState,
    services::rpc_service_async::{CurrentHeadHeader, CurrentHeadMetadata, NetworkConstants},
    synchronization::SynchronizationState,
    terminal_ui::UiState,
};

#[derive(Debug, Clone)]
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

    pub log: Logger,
}

impl State {
    pub fn new(baker_address: Option<String>, log: Logger) -> Self {
        Self {
            log,
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
        }
    }
}
