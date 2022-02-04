use hdrhistogram::Histogram;

use serde::Deserialize;

use crate::extensions::convert_time_to_unit_string;

pub type PerPeerBlockStatisticsVector = Vec<PerPeerBlockStatistics>;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BlockApplicationStatistics {
    pub block_hash: String,
    pub block_timestamp: u64,
    pub receive_timestamp: u64,
    pub baker: Option<String>,
    pub baker_priority: Option<u16>,
    pub download_block_header_start: Option<u64>,
    pub download_block_header_end: Option<u64>,
    pub download_block_operations_start: Option<u64>,
    pub download_block_operations_end: Option<u64>,
    pub load_data_start: Option<u64>,
    pub load_data_end: Option<u64>,
    pub apply_block_start: Option<u64>,
    pub apply_block_end: Option<u64>,
    pub store_result_start: Option<u64>,
    pub store_result_end: Option<u64>,
    pub protocol_times: Option<BlockApplicationProtocolStatistics>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BlockApplicationProtocolStatistics {
    pub apply_start: u64,
    pub operations_decoding_start: u64,
    pub operations_decoding_end: u64,
    // pub operations_application: Vec<Vec<(u64, u64)>>,
    pub operations_metadata_encoding_start: u64,
    pub operations_metadata_encoding_end: u64,
    pub begin_application_start: u64,
    pub begin_application_end: u64,
    pub finalize_block_start: u64,
    pub finalize_block_end: u64,
    pub collect_new_rolls_owner_snapshots_start: u64,
    pub collect_new_rolls_owner_snapshots_end: u64,
    pub commit_start: u64,
    pub commit_end: u64,
    pub apply_end: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PerPeerBlockStatistics {
    pub address: String, // TODO: url?
    pub block_hash: String,
    pub node_id: String,
    pub received_time: u64,
    pub sent_time: u64,
}

#[derive(Debug, Default, Clone)]
pub struct BakingState {
    pub application_statistics: Vec<BlockApplicationStatistics>,
    pub per_peer_block_statistics: PerPeerBlockStatisticsVector,
}

pub trait ToHistogramData {
    fn to_histogram_data(&self) -> Vec<(String, u64)>;
}

// TODO: we can use generic here?
impl ToHistogramData for PerPeerBlockStatisticsVector {
    fn to_histogram_data(&self) -> Vec<(String, u64)> {
        // TODO: error handling
        let mut histogram = Histogram::<u64>::new(1).unwrap();

        for stats in self.iter() {
            histogram.record(stats.received_time).unwrap()
        }
        // 100_000_000

        histogram
            .iter_linear(100_000_000)
            .map(|iter_val| {
                (
                    convert_time_to_unit_string(iter_val.value_iterated_to()),
                    iter_val.count_since_last_iteration(),
                )
            })
            .collect()

        // BTreeMap::new()
    }
}
