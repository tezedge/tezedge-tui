use std::{collections::BTreeMap, sync::Arc};

use slog::error;
use std::sync::RwLock;
use strum_macros::{Display, EnumIter};
use tui::widgets::{Table, TableState};

use crate::node_rpc::{Node, RpcCall, RpcResponse};

use super::{
    BlockApplicationStatus, BlockMetrics, ChainStatus, CurrentHeadHeader, Cycle, EndorsementRights,
    EndorsementState, EndorsementStatus, EndorsementStatusSortable, EndorsementStatusSortableVec,
    IncomingTransferMetrics, OperationsStats, OperationsStatsSortable, PeerMetrics, PeerTableData,
    SortableByFocus,
};

pub type StateRef = Arc<RwLock<State>>;

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

    // TODO: make it sortable
    pub operations_statistics: (OperationsStats, OperationsStatsSortable),
    pub statistics_pending: bool,
}

impl State {
    pub fn update_incoming_transfer(&mut self, incoming: IncomingTransferMetrics) {
        self.incoming_transfer = incoming;
    }

    pub fn update_application_status(&mut self, application_status: BlockApplicationStatus) {
        if let Some(last_appliead_block) = &application_status.last_applied_block {
            self.last_applied_level = last_appliead_block.level;
        }
        self.aplication_status = application_status;
    }

    pub fn update_peer_metrics(&mut self, peer_metrics: Vec<PeerMetrics>) {
        let table_data: PeerTableData = peer_metrics
            .into_iter()
            .map(|metrics| metrics.to_table_representation())
            .collect();
        self.peer_metrics = table_data;
    }

    pub fn update_block_metrics(&mut self, block_metrics: Vec<BlockMetrics>) {
        self.block_metrics = block_metrics;
    }

    pub fn update_cycle_data(&mut self, chain_status: ChainStatus) {
        self.cycle_data = chain_status.chain;
    }

    pub async fn update_current_head_header(&mut self, node: &Node, sort_by: usize) {
        match node.call_rpc(RpcCall::CurrentHeadHeader, None).await {
            Ok(RpcResponse::CurrentHeadHeader(header)) => {
                // only update the head and rights on head change
                if header.level != self.current_head_header.level {
                    self.current_head_header = header;
                    self.update_head_endorsing_rights(node).await;
                    self.reset_endorsers(sort_by);
                }
            }
            Err(e) => {
                error!(node.log, "{}", e);
            }
            _ => {}
        };
    }

    async fn update_head_endorsing_rights(&mut self, node: &Node) {
        let block_hash = &self.current_head_header.hash;
        let block_level = self.current_head_header.level;

        match node
            .call_rpc(
                RpcCall::EndorsementRights,
                Some(&format!("?block={}&level={}", block_hash, block_level)),
            )
            .await
        {
            Ok(RpcResponse::EndorsementRights(rights)) => {
                self.endorsement_rights = rights;
            }
            Err(e) => {
                error!(node.log, "{}", e);
            }
            _ => {}
        };
    }

    fn reset_endorsers(&mut self, sort_by: usize) {
        let mut statuses: EndorsementStatusSortableVec = self
            .endorsement_rights
            .iter()
            .map(|(k, slots)| EndorsementStatusSortable::new(k.to_string(), slots.len()))
            .collect();

        statuses.sort_by_focus(sort_by, false); // TODO

        self.current_head_endorsement_statuses = statuses;
    }

    pub async fn update_endorsers(&mut self, node: &Node, sort_by: usize) {
        let slot_mapped: BTreeMap<u32, EndorsementStatus> =
            match node.call_rpc(RpcCall::EndersementsStatus, None).await {
                Ok(RpcResponse::EndorsementsStatus(statuses)) => {
                    // build a per slot representation to be used later
                    statuses.into_iter().map(|(_, v)| (v.slot, v)).collect()
                }
                Err(e) => {
                    error!(node.log, "{}", e);
                    BTreeMap::new()
                }
                _ => BTreeMap::new(),
            };

        if !slot_mapped.is_empty() {
            // TODO: we need some info for the slots
            // if let Some((_, status)) = slot_mapped.iter().last() {
            //     if let Ok(time) =
            //         chrono::DateTime::parse_from_rfc3339(&self.current_head_header.timestamp)
            //     {
            //         // TODO: investigate this cast
            //         if status.block_timestamp != time.timestamp_nanos() as u64 {
            //             return;
            //         }
            //     } else {
            //         return;
            //     };
            // }

            let mut sumary: BTreeMap<EndorsementState, usize> = BTreeMap::new();

            let mut endorsement_operation_time_statistics: EndorsementStatusSortableVec = self
                .endorsement_rights
                .iter()
                .map(|(k, v)| {
                    if let Some((_, status)) = slot_mapped.iter().find(|(slot, _)| v.contains(slot))
                    {
                        let status = status.to_sortable(k.to_string(), v.len());
                        let state_count = sumary.entry(status.state.clone()).or_insert(0);
                        *state_count += status.slot_count;
                        status
                    } else {
                        let status = EndorsementStatusSortable::new(k.to_string(), v.len());
                        let state_count = sumary.entry(EndorsementState::Missing).or_insert(0);
                        *state_count += status.slot_count;
                        status
                    }
                })
                .collect();

            endorsement_operation_time_statistics.sort_by_focus(sort_by, false); // TODO

            self.current_head_endorsement_statuses = endorsement_operation_time_statistics;
            self.endoresement_status_summary = sumary;
        }
    }

    // Leaving this as an associated so we won't block by aquiring th write lock
    pub async fn update_statistics(
        node: &Node,
        sort_focus: usize,
        delta_toggle: bool,
    ) -> (OperationsStats, OperationsStatsSortable) {
        match node.call_rpc(RpcCall::OperationsStats, None).await {
            Ok(RpcResponse::OperationsStats(stats)) => {
                let mut sortable: OperationsStatsSortable = stats
                    .clone()
                    .into_iter()
                    .map(|(k, v)| v.to_statistics_sortable(k))
                    .collect();

                sortable.sort_by_focus(sort_focus, delta_toggle);
                (
                    stats.clone(),
                    stats
                        .into_iter()
                        .map(|(k, v)| v.to_statistics_sortable(k))
                        .collect(),
                )
            }
            Err(e) => {
                error!(node.log, "{}", e);
                (BTreeMap::new(), Vec::new())
            }
            _ => (BTreeMap::new(), Vec::new()),
        }
    }
}

/// TUI statefull widget states
#[derive(Debug, Clone, Default)]
pub struct UiState {
    pub peer_table_state: TableState,
    pub period_info_state: PeriodInfoState,
    pub endorsement_table_state: TableState,
    pub active_page: ActivePage,
    pub active_widget: ActiveWidget,

    pub main_operation_statistics_table_state: TableState,
    pub details_operation_statistics_table_state: TableState,
    pub main_operation_statistics_table_roller_state: RollableTableState,
    pub current_details_length: usize,
    pub delta_toggle: bool,
}

impl UiState {
    pub fn new() -> UiState {
        UiState {
            main_operation_statistics_table_roller_state: RollableTableState::new(3, 13),
            delta_toggle: true,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PeriodInfoState {
    pub container_count: usize,
    pub displayable_container_count: usize,
    pub selected: Option<usize>,
    offset: usize,
}

impl PeriodInfoState {
    pub fn select(&mut self, selected: Option<usize>) {
        self.selected = selected;
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn offset(&self) -> usize {
        if let Some(selected) = self.selected {
            if selected >= self.displayable_container_count {
                selected - self.displayable_container_count
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
    Unsorted,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Unsorted
    }
}

#[derive(Clone, Debug, Default)]
pub struct RollableTableState {
    /// Total number of indexex able to be rendered
    rendered: usize,

    /// Always render content this number of content starting from index 0
    fixed_count: usize,

    /// First index to be rendered after the last fixed index
    first_rendered_index: usize,

    /// The total number of columns
    total: usize,

    /// selected table column
    selected: usize,

    /// The index the table is sorted by
    sorted_by: Option<usize>,

    /// Sort order
    sort_order: SortOrder,
}

impl RollableTableState {
    pub fn new(fixed_count: usize, total: usize) -> Self {
        Self {
            fixed_count,
            rendered: 0,
            first_rendered_index: fixed_count,
            total,
            selected: 0,
            sorted_by: None,
            sort_order: SortOrder::Unsorted,
        }
    }

    pub fn sorted_by(&self) -> Option<usize> {
        self.sorted_by
    }

    pub fn sort_order(&self) -> &SortOrder {
        &self.sort_order
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn rendered(&self) -> usize {
        self.rendered
    }

    pub fn fixed(&self) -> usize {
        self.fixed_count
    }

    pub fn first_rendered_index(&self) -> usize {
        self.first_rendered_index
    }

    pub fn set_first_rendered_index(&mut self, first_rendered_index: usize) {
        self.first_rendered_index = first_rendered_index;
    }

    pub fn set_rendered(&mut self, rendered: usize) {
        self.rendered = rendered
    }

    pub fn set_fixed(&mut self, fixed: usize) {
        self.fixed_count = fixed
    }

    pub fn set_sort_order(&mut self, sort_order: SortOrder) {
        self.sort_order = sort_order
    }

    pub fn set_sorted_by(&mut self, sorted_by: Option<usize>) {
        self.sorted_by = sorted_by
    }

    pub fn next(&mut self) {
        let last_render_index = self.first_rendered_index + (self.rendered - self.fixed_count) - 1;
        let next_index = self.selected + 1;
        if next_index < self.total {
            self.selected = next_index
        }

        if self.selected >= last_render_index
            && self.first_rendered_index != last_render_index
            && self.rendered != self.total
        {
            self.first_rendered_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected != 0 && self.selected != self.total {
            self.selected -= 1;
        }

        if self.selected == self.first_rendered_index - 1
            && self.first_rendered_index != self.fixed_count
            && self.rendered != self.total
        {
            self.first_rendered_index -= 1;
        }
    }
}

#[derive(Debug, Clone)]
pub enum ActiveWidget {
    PeriodInfo,
    PeerTable,
    EndorserTable,
    StatisticsMainTable,
    StatisticsDetailsTable,
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ActivePage {
    Synchronization,
    Mempool,
    Statistics,
}

impl ActivePage {
    pub fn to_index(&self) -> usize {
        match self {
            ActivePage::Synchronization => 0,
            ActivePage::Mempool => 1,
            ActivePage::Statistics => 2,
        }
    }
}

impl Default for ActivePage {
    fn default() -> Self {
        ActivePage::Synchronization
    }
}

impl Default for ActiveWidget {
    fn default() -> Self {
        ActiveWidget::PeriodInfo
    }
}
