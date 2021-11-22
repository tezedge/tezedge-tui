use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use serde::Deserialize;
use std::sync::RwLock;
use tui::widgets::{Paragraph, TableState};

pub type StateRef = Arc<RwLock<State>>;
pub type PeerTableData = Vec<[String; 4]>;

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
}

/// TUI statefull widget states
#[derive(Debug, Clone, Default)]
pub struct UiState {
    pub peer_table_state: TableState,
    pub period_info_state: PeriodInfoState,
    pub pages: PageState,
}

#[derive(Debug, Clone)]
pub struct PageState {
    pub titles: Vec<String>,
    pub index: usize,
    pub widget_state: Vec<WidgetState>,
}

impl Default for PageState {
    fn default() -> Self {
        let syncing_widgets =
            WidgetState::new(["periods".to_string(), "peers".to_string()].to_vec());
        Self {
            titles: ["syncing".to_string(), "mempool".to_string()].to_vec(),
            index: 0,
            widget_state: vec![syncing_widgets],
        }
    }
}

impl PageState {
    pub fn in_focus(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    pub titles: Vec<String>,
    index: usize,
}

impl WidgetState {
    pub fn new(titles: Vec<String>) -> Self {
        Self { titles, index: 0 }
    }

    pub fn in_focus(&self) -> usize {
        self.index
    }
}

impl RollingList for WidgetState {
    fn get_mutable_index(&mut self) -> &mut usize {
        &mut self.index
    }

    fn get_titles(&self) -> &Vec<String> {
        &self.titles
    }
}

impl PageState {
    pub fn new(titles: Vec<String>, widget_state: Vec<WidgetState>) -> Self {
        Self {
            titles,
            index: 0,
            widget_state,
        }
    }
}

impl RollingList for PageState {
    fn get_mutable_index(&mut self) -> &mut usize {
        &mut self.index
    }

    fn get_titles(&self) -> &Vec<String> {
        &self.titles
    }
}

pub trait RollingList {
    fn get_mutable_index(&mut self) -> &mut usize;
    fn get_titles(&self) -> &Vec<String>;

    fn next(&mut self) {
        let titles = self.get_titles().clone();

        let index = self.get_mutable_index();
        *index = (*index + 1) % titles.len();
    }

    fn previous(&mut self) {
        let titles = self.get_titles().clone();
        let index = self.get_mutable_index();

        if *index > 0 {
            *index -= 1;
        } else {
            *index = titles.len() - 1;
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
