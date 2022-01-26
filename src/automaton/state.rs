use slog::Logger;

use crate::{
    endorsements::EndrosementsState, operations::OperationsStatisticsState, rpc::RpcState,
    services::rpc_service::CurrentHeadHeader, synchronization::SynchronizationState,
    terminal_ui::UiState,
};

#[derive(Debug, Clone)]
pub struct State {
    pub last_applied_level: i32,
    pub current_head_header: CurrentHeadHeader,

    pub synchronization: SynchronizationState,
    pub endorsmenents: EndrosementsState,
    pub operations_statistics: OperationsStatisticsState,

    pub delta_toggle: bool,

    pub rpc_state: RpcState,

    pub ui: UiState,

    pub log: Logger,
}

impl State {
    pub fn new(log: Logger) -> Self {
        Self {
            log,
            delta_toggle: true,
            current_head_header: Default::default(),
            last_applied_level: Default::default(),
            synchronization: Default::default(),
            endorsmenents: Default::default(),
            operations_statistics: Default::default(),
            rpc_state: Default::default(),
            ui: Default::default(),
        }
    }
}
