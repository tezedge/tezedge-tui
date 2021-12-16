use std::{
    collections::{BTreeMap}, sync::Arc, time::Duration,
};

use std::sync::RwLock;
use tui::widgets::TableState;

use crate::node_rpc::{Node, RpcCall, RpcResponse};

use super::{IncomingTransferMetrics, BlockApplicationStatus, PeerTableData, BlockMetrics, Cycle, CurrentHeadHeader, EndorsementRights, EndorsementRightsTableData, EndorsementState, PeerMetrics, ChainStatus, EndorsementStatus, EndorsementStatusSortable};

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
    pub current_head_endorsement_statuses: EndorsementRightsTableData,
    pub endoresement_status_summary: BTreeMap<EndorsementState, usize>,
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

    pub async fn update_current_head_header(&mut self, node: &Node) {
        if let Ok(RpcResponse::CurrentHeadHeader(header)) =
            node.call_rpc(RpcCall::CurrentHeadHeader, None).await
        {
            // only update the head and rights on head change
            if header.level != self.current_head_header.level {
                self.current_head_header = header;
                self.update_head_endorsing_rights(node).await;
                self.reset_endorsers();
            }
        };
    }

    async fn update_head_endorsing_rights(&mut self, node: &Node) {
        let block_hash = &self.current_head_header.hash;
        let block_level = self.current_head_header.level;

        if let Ok(RpcResponse::EndorsementRights(rights)) = node
            .call_rpc(
                RpcCall::EndorsementRights,
                Some(&format!("?block={}&level={}", block_hash, block_level)),
            )
            .await
        {
            self.endorsement_rights = rights;
        };
    }

    fn reset_endorsers(&mut self) {
        self.current_head_endorsement_statuses = self.endorsement_rights.iter().map(|(k, slots)| {
            EndorsementStatusSortable::new(k.to_string(), slots.len()).construct_tui_table_data()
        })
        .collect();
    }

    pub async fn update_endorsers(&mut self, node: &Node) {

        let slot_mapped: BTreeMap<u32, EndorsementStatus> = if let Ok(RpcResponse::EndorsementsStatus(statuses)) =
            node.call_rpc(RpcCall::EndersementsStatus, None).await
        {
            // build a per slot representation to be used later
            statuses.into_iter().map(|(_, v)| (v.slot, v)).collect()
        } else {
            BTreeMap::new()
        };

        if !slot_mapped.is_empty() {
            if let Some((_, status)) = slot_mapped.iter().last() {
                if let Ok(time) = chrono::DateTime::parse_from_rfc3339(&self.current_head_header.timestamp) {
                    // TODO: investigate this cast
                    if status.block_timestamp != time.timestamp_nanos() as u64 {
                        return;
                    }
                } else {
                    return;
                };
            }

            let mut sumary: BTreeMap<EndorsementState, usize> = BTreeMap::new();

            let mut endorsement_operation_time_statistics: Vec<EndorsementStatusSortable> = self.endorsement_rights
                .iter()
                .map(|(k, v)| {
                    if let Some((_, status)) = slot_mapped.iter().find(|(slot, _)| v.contains(slot)) {
                        status.to_sortable_ascending(k.to_string(), v.len())
                    } else {
                        EndorsementStatusSortable::new(k.to_string(), v.len())
                    }
                })
                .collect();

            endorsement_operation_time_statistics.sort_by_key(|k| k.delta);

            let table_data: EndorsementRightsTableData = endorsement_operation_time_statistics
                .into_iter()
                .map(|v| {
                    let state_count = sumary.entry(v.state.clone()).or_insert(0);
                    *state_count += v.slot_count;
                    v.construct_tui_table_data()
                })
                .collect();

            self.current_head_endorsement_statuses = table_data;
            self.endoresement_status_summary = sumary;
        }
    }
}

/// TUI statefull widget states
#[derive(Debug, Clone, Default)]
pub struct UiState {
    pub peer_table_state: TableState,
    pub period_info_state: PeriodInfoState,
    pub page_state: PageState,
}

#[derive(Debug, Clone)]
pub struct PageState {
    pub pages: Vec<Page>,
    pub in_focus: usize,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub widgets: WidgetState,
}

impl Page {
    fn new(title: String, widgets: WidgetState) -> Self {
        Self { title, widgets }
    }
}

impl Default for PageState {
    fn default() -> Self {
        Self {
            pages: vec![
                Page::new(
                    "Synchronization".to_string(),
                    WidgetState::new(vec!["Periods".to_string(), "Connected peers".to_string()]),
                ),
                Page::new(
                    "Mempool".to_string(),
                    WidgetState::new(vec!["TODOWidget".to_string()]),
                ),
            ],
            in_focus: 0,
        }
    }
}

impl PageState {
    pub fn in_focus(&self) -> usize {
        self.in_focus
    }
}

#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    widgets: Vec<PageWidget>,
    in_focus: usize,
}

pub type PageWidget = String;

impl WidgetState {
    fn new(widgets: Vec<PageWidget>) -> Self {
        Self {
            widgets,
            in_focus: 0,
        }
    }

    pub fn in_focus(&self) -> usize {
        self.in_focus
    }
}

impl RollingList for WidgetState {
    fn get_mutable_in_focus(&mut self) -> &mut usize {
        &mut self.in_focus
    }

    fn get_count(&self) -> usize {
        self.widgets.len()
    }
}

// impl PageState {
//     pub fn new(titles: Vec<String>, widget_state: Vec<WidgetState>) -> Self {
//         Self {
//             titles,
//             index: 0,
//             widget_state,
//         }
//     }
// }

impl RollingList for PageState {
    fn get_mutable_in_focus(&mut self) -> &mut usize {
        &mut self.in_focus
    }

    fn get_count(&self) -> usize {
        self.pages.len()
    }
}

pub trait RollingList {
    fn get_mutable_in_focus(&mut self) -> &mut usize;
    fn get_count(&self) -> usize;

    fn next(&mut self) {
        let count = self.get_count();
        if count <= 1 {
            return;
        }

        let in_focus = self.get_mutable_in_focus();
        *in_focus = (*in_focus + 1) % count;
    }

    fn previous(&mut self) {
        let count = self.get_count();
        let in_focus = self.get_mutable_in_focus();

        if count <= 1 {
            return;
        }

        if *in_focus > 0 {
            *in_focus -= 1;
        } else {
            *in_focus = count - 1;
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