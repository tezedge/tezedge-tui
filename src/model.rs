use std::{collections::{BTreeMap, HashMap}, sync::Arc, time::Duration, hash::Hash};

use conv::TryFrom;
use itertools::Itertools;
use serde::Deserialize;
use std::sync::RwLock;
use tui::widgets::TableState;

use crate::node_rpc::{Node, RpcCall, RpcResponse};

pub type StateRef = Arc<RwLock<State>>;
pub type PeerTableData = Vec<[String; 4]>;
pub type EndorsementRights = BTreeMap<String, Vec<u32>>;
pub type EndorsementStatuses = BTreeMap<String, EndorsementStatus>;

// TODO: update accordingly
pub type EndorsementRightsTableData = Vec<Vec<String>>;

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
    pub current_head_endorsement_rights: EndorsementRightsTableData,
    pub endoresement_status_summary: HashMap<String, u16>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct EndorsementStatus {
    pub block_timestamp: u64,
    pub decoded_time: Option<u64>,
    pub received_time: Option<u64>,
    pub applied_time: Option<u64>,
    pub prechecked_time: Option<u64>,
    pub broadcast_time: Option<u64>,
    pub slot: u32,
    pub state: String,
}

#[derive(Clone, Debug, Default)]
pub struct EndorsementStatusSortable {
    pub delta: u64,
    pub decoded_time: u64,
    pub received_time: u64,
    pub applied_time: u64,
    pub prechecked_time: u64,
    pub broadcast_time: u64,
    pub slot: u32,
    pub state: String,

    pub baker: String,
    pub slot_count: usize,
}

impl EndorsementStatus {
    pub fn to_sortable_descending(&self, baker: String, slot_count: usize) -> EndorsementStatusSortable {
        let delta = if let (Some(broadcast), Some(received)) = (self.broadcast_time, self.received_time) {
            broadcast - received
        } else {
            0
        };

        let received_time = if let Some(received) = self.received_time {
            received
        } else {
            0
        };

        let decoded_time = if let Some(decoded) = self.decoded_time {
            decoded
        } else {
            0
        };

        let prechecked_time = if let Some(prechecked) = self.prechecked_time {
            prechecked
        } else {
            0
        };

        let applied_time = if let Some(applied) = self.applied_time {
            applied
        } else {
            0
        };

        let broadcast_time = if let Some(broadcast) = self.broadcast_time {
            broadcast
        } else {
            0
        };

        EndorsementStatusSortable {
            baker,
            slot_count,
            delta,
            received_time,
            decoded_time,
            prechecked_time,
            applied_time,
            broadcast_time,
            slot: self.slot,
            state: self.state.clone(),
        }
    }

    pub fn to_sortable_ascending(&self, baker: String, slot_count: usize) -> EndorsementStatusSortable {
        let delta = if let (Some(broadcast), Some(received)) = (self.broadcast_time, self.received_time) {
            broadcast - received
        } else {
            u64::MAX
        };

        let received_time = if let Some(received) = self.received_time {
            received
        } else {
            u64::MAX
        };

        let decoded_time = if let Some(decoded) = self.decoded_time {
            decoded
        } else {
            u64::MAX
        };

        let prechecked_time = if let Some(prechecked) = self.prechecked_time {
            prechecked
        } else {
            u64::MAX
        };

        let applied_time = if let Some(applied) = self.applied_time {
            applied
        } else {
            u64::MAX
        };

        let broadcast_time = if let Some(broadcast) = self.broadcast_time {
            broadcast
        } else {
            u64::MAX
        };

        EndorsementStatusSortable {
            baker,
            slot_count,
            delta,
            received_time,
            decoded_time,
            prechecked_time,
            applied_time,
            broadcast_time,
            slot: self.slot,
            state: self.state.clone(),
        }
    }
}

impl EndorsementStatusSortable {
    fn new(baker: String) -> Self {
        Self {
            baker,
            state: "missing".to_string(),
            ..Default::default()
        }
    }

    pub fn construct_tui_table_data(&self) -> Vec<String> {
        let mut final_vec = Vec::with_capacity(9);

        final_vec.push(self.slot_count.to_string());
        final_vec.push(self.baker.clone());
        final_vec.push(self.state.clone());

        if self.broadcast_time != 0 && self.received_time != 0 && self.broadcast_time != u64::MAX && self.received_time != u64::MAX{
            final_vec.push(convert_time_to_unit_string(self.broadcast_time - self.received_time))
        } else {
            final_vec.push(String::from('-'));
        }

        if self.received_time > 0 && self.received_time != u64::MAX {
            final_vec.push(convert_time_to_unit_string(self.received_time));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.decoded_time > 0 && self.decoded_time != u64::MAX {
            final_vec.push(convert_time_to_unit_string(self.decoded_time));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.prechecked_time > 0 && self.prechecked_time != u64::MAX {
            final_vec.push(convert_time_to_unit_string(self.prechecked_time));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.applied_time > 0 && self.applied_time != u64::MAX {
            final_vec.push(convert_time_to_unit_string(self.applied_time));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.broadcast_time > 0 && self.broadcast_time != u64::MAX {
            final_vec.push(convert_time_to_unit_string(self.broadcast_time));
        } else {
            final_vec.push(String::from('-'));
        }

        final_vec
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CurrentHeadHeader {
    pub level: i32,
    pub hash: String,
    pub timestamp: String,
    pub chain_id: String,
    pub predecessor: String,
    pub validation_pass: u8,
    pub operations_hash: String,
    pub fitness: Vec<String>,
    pub context: String,
    pub protocol: String,
    pub signature: String,
    pub priority: i32,
    pub proof_of_work_nonce: String,
    pub liquidity_baking_escape_vote: bool,
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

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct IncomingTransferMetrics {
    pub eta: Option<f32>,
    pub current_block_count: usize,
    pub downloaded_blocks: usize,
    pub download_rate: f32,
    pub average_download_rate: f32,
    pub downloaded_headers: usize,
    pub header_download_rate: f32,
    pub header_average_download_rate: f32,
}

#[derive(Clone, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockApplicationStatus {
    pub current_application_speed: f32,
    pub average_application_speed: f32,
    pub last_applied_block: Option<BlockInfo>,
}

#[derive(Clone, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub hash: String,
    pub level: i32,
}

#[derive(Clone, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PeerMetrics {
    id: String,
    ip_address: String,
    transferred_bytes: usize,
    average_transfer_speed: f32,
    current_transfer_speed: f32,
}

impl PeerMetrics {
    pub fn to_table_representation(&self) -> [String; 4] {
        [
            self.ip_address.to_string(),
            self.transferred_bytes.to_string(),
            self.average_transfer_speed.to_string(),
            self.current_transfer_speed.to_string(),
        ]
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockMetrics {
    pub group: i32,
    pub numbers_of_blocks: i32,
    pub finished_blocks: i32,
    pub applied_blocks: i32,
    pub download_duration: Option<f32>,
}

impl BlockMetrics {
    pub fn all_downloaded(&self) -> bool {
        self.finished_blocks >= self.numbers_of_blocks
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cycle {
    // cycle id
    pub id: usize,
    // number of downloaded block headers per cycle
    pub headers: usize,
    // number of downloaded block operatios per cycle
    pub operations: usize,
    // number of applied blocks
    pub applications: usize,
    // time to download headers and operations for cycle
    pub duration: Option<f32>,
}

impl Cycle {
    pub fn all_applied(&self) -> bool {
        // when we see Some(duration) instead of None, all the headers are downloaded
        if self.duration.is_some() {
            // if the number of headers is the same as the number of applications, all blocks in the cycle are appliead
            self.applications == self.headers
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainStatus {
    pub chain: Vec<Cycle>,
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
        self.current_head_header = if let Ok(RpcResponse::CurrentHeadHeader(header)) =
            node.call_rpc(RpcCall::CurrentHeadHeader, None).await
        {
            header
        } else {
            CurrentHeadHeader::default()
        }
    }

    pub async fn update_endorsers(&mut self, node: &Node) {
        let block_hash = &self.current_head_header.hash;
        let block_level = self.current_head_header.level;

        let rights = if let Ok(RpcResponse::EndorsementRights(rights)) = node
            .call_rpc(
                RpcCall::EndorsementRights,
                Some(&format!("?block={}&level={}", block_hash, block_level)),
            )
            .await
        {
            rights
        } else {
            EndorsementRights::default()
        };

        let statuses = if let Ok(RpcResponse::EndorsementsStatus(statuses)) =
            node.call_rpc(RpcCall::EndersementsStatus, None).await
        {
            statuses
        } else {
            EndorsementStatuses::default()
        };

        // TODO: can be combined with the code above
        // build a per slot representation to be used later
        let slot_mapped: BTreeMap<u32, EndorsementStatus> =
            statuses.into_iter().map(|(_, v)| (v.slot, v)).collect();
        
        let mut sumary: HashMap<String, u16> = HashMap::new();

        // let endorsement_operation_statistics: BTreeMap<String, EndorsementStatus> = BTreeMap::new();

        let mut endorsement_operation_time_statistics: Vec<EndorsementStatusSortable> = rights
            .into_iter()
            .map(|(k, v)| {
                if let Some((_, status)) = slot_mapped.iter().find(|(slot, _)| v.contains(slot)) {
                    status.to_sortable_ascending(k, v.len())
                } else {
                    EndorsementStatusSortable::new(k)
                }
            })
            .collect();

        endorsement_operation_time_statistics.sort_by_key(|k| k.delta);

        let table_data: EndorsementRightsTableData = endorsement_operation_time_statistics
            .into_iter()
            .map(|v| v.construct_tui_table_data())
            .collect();


        self.current_head_endorsement_rights = table_data;
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

pub fn convert_time_to_unit_string(time: u64) -> String {
    let time = time as f64;
    const MILLISECOND_FACTOR: f64 = 1000.0;
    const MICROSECOND_FACTOR: f64 = 1000000.0;
    const NANOSECOND_FACTOR: f64 = 1000000000.0;

    if time >= NANOSECOND_FACTOR {
        format!("{:.2}s", time / NANOSECOND_FACTOR)
    } else if time >= MICROSECOND_FACTOR {
        format!("{:.2}ms", time / MICROSECOND_FACTOR)
    } else if time >= MILLISECOND_FACTOR {
        format!("{:.2}μs", time / MILLISECOND_FACTOR)
    } else {
        format!("{}ns", time)
    }
}
