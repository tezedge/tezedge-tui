use serde::{Deserialize, Serialize};
use tui::widgets::TableState;

pub type PeerTableData = Vec<[String; 4]>;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SynchronizationState {
    // info for the syncing and apllication blocks
    pub incoming_transfer: IncomingTransferMetrics,
    pub aplication_status: BlockApplicationStatus,
    // info for the peer table on syncing screen
    pub peer_metrics: Vec<PeerMetrics>,

    // info for the period blocks
    pub block_metrics: Vec<BlockStatus>,
    pub cycle_data: Vec<Cycle>,

    // ui specific states
    #[serde(skip)]
    pub peer_table_state: TableState,
    pub period_info_state: PeriodInfoState,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
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

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockApplicationStatus {
    pub current_application_speed: f32,
    pub average_application_speed: f32,
    pub last_applied_block: Option<BlockInfo>,
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub hash: String,
    pub level: i32,
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStatus {
    pub group: i32,
    pub numbers_of_blocks: i32,
    pub finished_blocks: i32,
    pub applied_blocks: i32,
    pub download_duration: Option<f32>,
}

impl BlockStatus {
    pub fn all_downloaded(&self) -> bool {
        self.finished_blocks >= self.numbers_of_blocks
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainStatus {
    pub chain: Vec<Cycle>,
}
