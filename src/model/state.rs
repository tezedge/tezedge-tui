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
    pub endorsement_sorter_state: SorterState,
    pub endorsement_table_state: TableState,
    pub active_page: ActivePage,
    pub active_widget: ActiveWidget,

    pub main_operation_statistics_table_state: TableState,
    pub details_operation_statistics_table_state: TableState,
    pub main_operation_statistics_sorter_state: SorterState,
    pub details_operation_statistics_sorter_state: SorterState,
    pub main_operation_statistics_table_roller_state: RollableTableState,
    pub current_details_length: usize,
    pub delta_toggle: bool,
}

impl UiState {
    pub fn new() -> UiState {
        UiState {
            endorsement_sorter_state: SorterState::new(9, 3),
            main_operation_statistics_sorter_state: SorterState::new(13, 0),
            details_operation_statistics_sorter_state: SorterState::new(7, 0),
            main_operation_statistics_table_roller_state: RollableTableState::new(&[0, 1, 2, 3, 4]),
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
}

#[derive(Debug, Clone)]
pub struct SorterState {
    sorter_count: usize,
    in_focus: usize,
    pub order: SortOrder,
}

impl SorterState {
    pub fn new(sorter_count: usize, in_focus: usize) -> SorterState {
        SorterState {
            sorter_count,
            in_focus,
            order: SortOrder::Ascending,
            ..Default::default()
        }
    }
    pub fn in_focus(&self) -> usize {
        self.in_focus
    }

    pub fn next(&mut self) {
        let next_index = self.in_focus + 1;
        if next_index >= self.sorter_count {
            self.in_focus = 0
        } else {
            self.in_focus = next_index
        }
    }

    pub fn previous(&mut self) {
        if self.in_focus == 0 {
            self.in_focus = self.sorter_count;
        } else {
            self.in_focus -= 1;
        }
    }
}

impl Default for SorterState {
    fn default() -> Self {
        Self {
            in_focus: 0,
            sorter_count: 0,
            order: SortOrder::Ascending,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RollableTableState {
    /// Total number of indexex able to be rendered
    rendered: usize,

    /// Always render content at these indexes
    fixed: Vec<usize>,

    /// First index to be rendered
    first_rendered_index: usize,
}

impl RollableTableState {

    pub fn new(fixed: &[usize]) -> Self {
        Self {
            fixed: fixed.to_vec(),
            rendered: 0,
            first_rendered_index: fixed.len(),
        }
    }

    pub fn rendered(&self) -> usize {
        self.rendered
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

    pub fn set_fixed(&mut self, fixed: &[usize]) {
        self.fixed = fixed.to_vec()
    }

    pub fn next(&mut self) {
        let next_index = self.first_rendered_index + 1;
        if next_index < self.rendered {
            self.first_rendered_index = next_index
        }
    }

    pub fn previous(&mut self) {
        if self.first_rendered_index != 0 {
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
