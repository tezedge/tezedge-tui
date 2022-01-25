use crate::{
    endorsements::EndrosementsState, rpc::RpcState, services::rpc_service::CurrentHeadHeader,
    terminal_ui::UiState,
};

#[derive(Debug, Clone, Default)]
pub struct State {
    // info for the syncing and apllication blocks
    // pub incoming_transfer: IncomingTransferMetrics,
    // pub aplication_status: BlockApplicationStatus,
    pub last_applied_level: i32,

    // info for the peer table on syncing screen
    // pub peer_metrics: PeerTableData,

    // info for the period blocks
    // pub block_metrics: Vec<BlockMetrics>,
    // pub cycle_data: Vec<Cycle>,
    pub current_head_header: CurrentHeadHeader,
    pub endorsmenents: EndrosementsState,

    // pub operations_statistics: (OperationsStats, OperationsStatsSortable),
    // pub selected_operation_details: Option<Vec<OperationDetailSortable>>,
    pub statistics_pending: bool,

    pub delta_toggle: bool,

    pub rpc_state: RpcState,

    pub ui: UiState,
}

impl State {}
