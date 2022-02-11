use slog::Logger;

use crate::{
    baking::BakingState, endorsements::EndrosementsState, operations::OperationsStatisticsState,
    services::rpc_service::CurrentHeadHeader, synchronization::SynchronizationState,
    terminal_ui::UiState,
};

#[derive(Debug, Clone)]
pub struct State {
    pub last_applied_level: i32,
    pub current_head_header: CurrentHeadHeader,
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
            last_applied_level: Default::default(),
            synchronization: Default::default(),
            endorsmenents: Default::default(),
            operations_statistics: Default::default(),
            baking: Default::default(),
            ui: Default::default(),
        }
    }
}
