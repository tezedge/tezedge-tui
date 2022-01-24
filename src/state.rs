
#[derive(Debug, Clone, Default)]
pub struct State {
    // info for the syncing and apllication blocks
    pub incoming_transfer: IncomingTransferMetrics,
    pub aplication_status: BlockApplicationStatus,
    pub last_applied_level: i32,

    // info for the peer table on syncing screen
    pub peer_metrics: PeerTableData,

    // info for the period blocks
    pub block_metrics: Vec<BlockMetrics>,
    pub cycle_data: Vec<Cycle>,
    pub current_head_header: CurrentHeadHeader,

    pub endorsement_rights: EndorsementRights,
    pub current_head_endorsement_statuses: EndorsementStatusSortableVec,
    pub endoresement_status_summary: BTreeMap<EndorsementState, usize>,

    pub operations_statistics: (OperationsStats, OperationsStatsSortable),
    pub selected_operation_details: Option<Vec<OperationDetailSortable>>,
    pub statistics_pending: bool,

    // TUI states - states refering to the view part

    pub peer_table_state: TableState,
    pub period_info_state: PeriodInfoState,
    pub endorsement_table: ExtendedTable,
    pub active_page: ActivePage,
    pub active_widget: ActiveWidget,

    pub details_operation_statistics_table: ExtendedTable,
    pub main_operation_statistics_table: ExtendedTable,
    pub current_details_length: usize,
    pub delta_toggle: bool,
}