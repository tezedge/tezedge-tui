use std::{collections::BTreeMap, sync::Arc};

use itertools::Itertools;
use slog::error;
use std::sync::RwLock;
use strum_macros::{Display, EnumIter};
use tui::{
    layout::Constraint,
    style::Style,
    widgets::{Cell, Row, TableState},
};

use crate::node_rpc::{Node, RpcCall, RpcResponse};

use super::{
    BlockApplicationStatus, BlockMetrics, ChainStatus, CurrentHeadHeader, Cycle, EndorsementRights,
    EndorsementState, EndorsementStatus, EndorsementStatusSortable, EndorsementStatusSortableVec,
    IncomingTransferMetrics, OperationDetailSortable, OperationsStats,
    OperationsStatsSortable, PeerMetrics, PeerTableData, SortableByFocus, TuiTableData,
};

pub type StateRef = Arc<RwLock<State>>;

const SIDE_PADDINGS: u16 = 1;
const INITIAL_PADDING: u16 = 2;
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

    pub async fn update_current_head_header(
        &mut self,
        node: &Node,
        sort_by: usize,
        sort_order: &SortOrder,
        delta_toggle: bool,
    ) {
        match node.call_rpc(RpcCall::CurrentHeadHeader, None).await {
            Ok(RpcResponse::CurrentHeadHeader(header)) => {
                // only update the head and rights on head change
                if header.level != self.current_head_header.level {
                    self.current_head_header = header;
                    self.update_head_endorsing_rights(node).await;
                    self.reset_endorsers(sort_by, sort_order, delta_toggle);
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

    fn reset_endorsers(&mut self, sort_by: usize, sort_order: &SortOrder, delta_toggle: bool) {
        let mut statuses: EndorsementStatusSortableVec = self
            .endorsement_rights
            .iter()
            .map(|(k, slots)| EndorsementStatusSortable::new(k.to_string(), slots.len()))
            .collect();

        statuses.sort_by_focus(sort_by, delta_toggle);
        if let SortOrder::Descending = *sort_order {
            statuses.reverse();
        }

        self.current_head_endorsement_statuses = statuses;
    }

    pub async fn update_endorsers(
        &mut self,
        node: &Node,
        sort_by: usize,
        sort_order: &SortOrder,
        delta_toggle: bool,
    ) {
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

            endorsement_operation_time_statistics.sort_by_focus(sort_by, delta_toggle);
            if let SortOrder::Descending = *sort_order {
                endorsement_operation_time_statistics.reverse();
            }

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

    pub fn update_selected_operation_details(&mut self, selected: Option<usize>) {
        if let Some(index) = selected {
            let hash = self.operations_statistics.1[index].hash.clone();

            if let Some(stats) = self.operations_statistics.0.get(&hash) {
                self.selected_operation_details = Some(stats.to_operations_details());
            }
        }
    }
}

/// TUI statefull widget states
#[derive(Debug, Clone, Default)]
pub struct UiState {
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

impl UiState {
    pub fn new() -> UiState {
        UiState {
            endorsement_table: ExtendedTable::new(
                vec![
                    "Slots",
                    "Baker",
                    "Status",
                    "Delta",
                    "Receive hash",
                    "Receive content",
                    "Decode",
                    "Precheck",
                    "Apply",
                    "Broadcast",
                ]
                .iter()
                .map(|v| v.to_string())
                .collect(),
                vec![
                    Constraint::Length(6),
                    Constraint::Length(36),
                    Constraint::Min(11),
                    Constraint::Min(8),
                    Constraint::Min(12),
                    Constraint::Min(15),
                    Constraint::Min(8),
                    Constraint::Min(9),
                    Constraint::Min(8),
                    Constraint::Min(10),
                ],
                4,
            ),
            main_operation_statistics_table: ExtendedTable::new(
                vec![
                    "Datetime",
                    "Hash",
                    "Nodes",
                    "Delta",
                    "Received",
                    "Content Received",
                    "Validation Started",
                    "Preapply Started",
                    "Preapply Finished",
                    "Validation Finished",
                    "Validation Length",
                    "Sent",
                    "Kind",
                ]
                .iter()
                .map(|v| v.to_string())
                .collect(),
                vec![
                    Constraint::Min(22),
                    Constraint::Min(9),
                    Constraint::Min(6),
                    Constraint::Min(9),
                    Constraint::Min(9),
                    Constraint::Min(17),
                    Constraint::Min(19),
                    Constraint::Min(17),
                    Constraint::Min(18),
                    Constraint::Min(20),
                    Constraint::Min(18),
                    Constraint::Min(9),
                    Constraint::Min(19),
                ],
                3,
            ),
            delta_toggle: true,
            details_operation_statistics_table: ExtendedTable::new(
                vec![
                    "Node Id", "1.Rec.", "1.Rec.C.", "1.Sent", "Received", "Con.Rec.", "Sent",
                ]
                .iter()
                .map(|v| v.to_string())
                .collect(),
                // TODO: expand for the sort symbol
                vec![
                    Constraint::Min(9),
                    Constraint::Min(9),
                    Constraint::Min(9),
                    Constraint::Min(9),
                    Constraint::Min(9),
                    Constraint::Min(9),
                    Constraint::Min(9),
                ],
                3,
            ),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
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
pub struct ExtendedTable {
    pub table_state: TableState,

    /// The header strings of the table in order
    headers: Vec<String>,

    modified_headers: Vec<String>,

    /// Constrainst of the colums
    constraints: Vec<Constraint>,

    /// Total number of indexex able to be rendered
    rendered: usize,

    /// Always render content this number of content starting from index 0
    fixed_count: usize,

    /// First index to be rendered after the last fixed index
    first_rendered_index: usize,

    /// selected table column
    selected: usize,

    /// The index the table is sorted by
    sorted_by: Option<usize>,

    /// Sort order
    sort_order: SortOrder,
}

impl ExtendedTable {
    pub fn new(headers: Vec<String>, constraints: Vec<Constraint>, fixed_count: usize) -> Self {
        Self {
            headers: headers.clone(),
            modified_headers: headers,
            constraints,
            fixed_count,
            rendered: 0,
            first_rendered_index: fixed_count,
            selected: 0,
            sorted_by: None,
            sort_order: SortOrder::Unsorted,
            ..Default::default()
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
        if next_index < self.headers.len() {
            self.selected = next_index
        }

        if self.selected >= last_render_index
            && self.first_rendered_index != last_render_index
            && self.rendered != self.headers.len()
        {
            self.first_rendered_index += 1;
        }
    }

    pub fn sort_content<T: SortableByFocus>(
        &mut self,
        content: &mut T,
        sort_by: usize,
        sort_order: &SortOrder,
        delta_toggle: bool,
    ) {
        self.sorted_by = Some(sort_by);
        self.sort_order = sort_order.clone();

        content.sort_by_focus(sort_by, delta_toggle);
        if let SortOrder::Descending = *sort_order {
            content.rev();
        }
    }

    pub fn previous(&mut self) {
        if self.selected != 0 && self.selected != self.headers.len() {
            self.selected -= 1;
        }

        if self.selected == self.first_rendered_index - 1
            && self.first_rendered_index != self.fixed_count
            && self.rendered != self.headers.len()
        {
            self.first_rendered_index -= 1;
        }
    }

    pub fn highlight_sorting(&mut self) {
        let mut headers = self.headers.clone();

        // add ▼/▲ to the selected sorted table
        if let Some(sorted_by) = self.sorted_by {
            if let Some(v) = headers.get_mut(sorted_by) {
                match self.sort_order {
                    SortOrder::Ascending => *v = format!("{}▲", v),
                    SortOrder::Descending => *v = format!("{}▼", v),
                    _ => {}
                }
            }
        }
        self.modified_headers = headers;
    }

    pub fn renderable_constraints(&mut self, max_size: u16) -> Vec<Constraint> {
        let mut acc: u16 = INITIAL_PADDING
            + self
                .constraints
                .iter()
                .take(self.fixed_count)
                .map(|c| {
                    if let Constraint::Min(unit) = c {
                        *unit
                    } else {
                        0
                    }
                })
                .reduce(|mut acc, unit| {
                    acc += unit;
                    acc
                })
                .unwrap_or(0);

        let mut to_render: Vec<Constraint> = self
            .constraints
            .iter()
            .take(self.fixed_count)
            .cloned()
            .collect();

        let dynamic_to_render: Vec<Constraint> = self
            .constraints
            .iter()
            .skip(self.first_rendered_index)
            .take_while_ref(|constraint| {
                if let Constraint::Min(unit) = constraint {
                    acc += unit + SIDE_PADDINGS;
                    acc <= max_size
                } else {
                    // TODO
                    false
                }
            })
            .cloned()
            .collect();

        to_render.extend(dynamic_to_render);

        self.rendered = to_render.len();
        to_render
    }

    pub fn renderable_headers(&self, selected_style: Style) -> Vec<Cell> {
        let selected = self.selected;
        let fixed_header_cells = self
            .modified_headers
            .iter()
            .enumerate()
            .take(self.fixed_count)
            .map(|(index, h)| {
                if index == selected {
                    Cell::from(h.as_str()).style(selected_style)
                } else {
                    Cell::from(h.as_str()).style(Style::default())
                }
            });

        let dynamic_header_cells = self
            .modified_headers
            .iter()
            .enumerate()
            .skip(self.first_rendered_index)
            .map(|(index, h)| {
                if index == selected {
                    Cell::from(h.as_str()).style(selected_style)
                } else {
                    Cell::from(h.as_str()).style(Style::default())
                }
            });

        fixed_header_cells.chain(dynamic_header_cells).collect()
    }

    pub fn renderable_rows<T: TuiTableData>(
        &self,
        content: &[T],
        delta_toggle: bool,
        selected_style: Style,
    ) -> Vec<Row> {
        let selected = self.selected;
        content
            .iter()
            .map(|item| {
                let item = item.construct_tui_table_data(delta_toggle);
                let height = item
                    .iter()
                    .map(|(content, _)| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;
                let fixed_cells = item.iter().enumerate().take(self.fixed_count).map(
                    |(index, (content, color))| {
                        if index == selected {
                            Cell::from(content.clone()).style(selected_style)
                        } else {
                            Cell::from(content.clone()).style(Style::default().fg(*color))
                        }
                    },
                );
                let dynamic_cells = item.iter().enumerate().skip(self.first_rendered_index).map(
                    |(index, (content, color))| {
                        if index == selected {
                            Cell::from(content.clone()).style(selected_style)
                        } else {
                            Cell::from(content.clone()).style(Style::default().fg(*color))
                        }
                    },
                );
                let cells = fixed_cells.chain(dynamic_cells);
                Row::new(cells).height(height as u16)
            })
            .collect()
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
