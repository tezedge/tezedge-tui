use hdrhistogram::Histogram;

use serde::Deserialize;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};

use crate::extensions::{convert_time_to_unit_string, convert_time_to_unit_string_option};

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
    pub precheck_start: Option<u64>,
    pub precheck_end: Option<u64>,
    pub apply_block_start: Option<u64>,
    pub apply_block_end: Option<u64>,
    pub store_result_start: Option<u64>,
    pub store_result_end: Option<u64>,
    pub send_start: Option<u64>,
    pub send_end: Option<u64>,
    pub protocol_times: Option<BlockApplicationProtocolStatistics>,
}

pub struct BlockApplicationSummary {
    pub precheck: Option<u64>,
    pub send_data: Option<u64>,
    pub download: Option<u64>,
    pub download_block_header: Option<u64>,
    pub download_block_operations: Option<u64>,
    pub load_data: Option<u64>,
    pub protocol_apply_block: Option<u64>,
    pub apply: Option<u64>,
    pub apply_begin_application: Option<u64>,
    pub apply_decoding_operations: Option<u64>,
    pub apply_encoding_operations_metadata: Option<u64>,
    pub apply_collecting_new_rolls: Option<u64>,
    pub apply_commit: Option<u64>,
    pub store_data: Option<u64>,
}

impl BlockApplicationSummary {
    pub fn to_table_data(&self) -> Vec<(Spans, String)> {
        let style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);
        vec![
            (
                Spans::from("Precheck"),
                convert_time_to_unit_string_option(self.precheck),
            ),
            (
                Spans::from("Send data"),
                convert_time_to_unit_string_option(self.send_data),
            ),
            (
                Spans::from("Download"),
                convert_time_to_unit_string_option(self.download),
            ),
            // start indention 1
            (
                Spans::from(vec![Span::styled("├─ ", style), Span::from("Block Header")]),
                convert_time_to_unit_string_option(self.download_block_header),
            ),
            // ("└─ Block Operations", 0u64),
            (
                Spans::from(vec![
                    Span::styled("└─ ", style),
                    Span::from("Block Operations"),
                ]),
                convert_time_to_unit_string_option(self.download_block_operations),
            ),
            // end indention 1
            (
                Spans::from("Load Data"),
                convert_time_to_unit_string_option(self.load_data),
            ),
            (
                Spans::from("Protocol Apply Block"),
                convert_time_to_unit_string_option(self.protocol_apply_block),
            ),
            // start indention 1
            (
                Spans::from(vec![Span::styled("└─ ", style), Span::from("Apply")]),
                convert_time_to_unit_string_option(self.apply),
            ),
            // start indendtion 2
            (
                Spans::from(vec![
                    Span::styled("   ├─ ", style),
                    Span::from("Begin application"),
                ]),
                convert_time_to_unit_string_option(self.apply_begin_application),
            ),
            (
                Spans::from(vec![
                    Span::styled("   ├─ ", style),
                    Span::from("Decoding operations"),
                ]),
                convert_time_to_unit_string_option(self.apply_decoding_operations),
            ),
            (
                Spans::from(vec![
                    Span::styled("   ├─ ", style),
                    Span::from("Encoding operations metadata"),
                ]),
                convert_time_to_unit_string_option(self.apply_encoding_operations_metadata),
            ),
            (
                Spans::from(vec![
                    Span::styled("   ├─ ", style),
                    Span::from("Collecting new rolls"),
                ]),
                convert_time_to_unit_string_option(self.apply_collecting_new_rolls),
            ),
            (
                Spans::from(vec![Span::styled("   └─ ", style), Span::from("Commit")]),
                convert_time_to_unit_string_option(self.apply_commit),
            ),
            // end indention 2
            // end indention 1
            (
                Spans::from("Store data"),
                convert_time_to_unit_string_option(self.store_data),
            ),
        ]
    }
}

impl From<BlockApplicationStatistics> for BlockApplicationSummary {
    fn from(stats: BlockApplicationStatistics) -> Self {
        let precheck = stats
            .precheck_start
            .and_then(|start| stats.precheck_end.map(|end| end - start));
        let send_data = stats
            .send_start
            .and_then(|start| stats.send_end.map(|end| end - start));
        let download = stats
            .download_block_header_start
            .and_then(|start| stats.download_block_operations_end.map(|end| end - start));
        let download_block_header = stats
            .download_block_header_start
            .and_then(|start| stats.download_block_header_end.map(|end| end - start));
        let download_block_operations = stats
            .download_block_operations_start
            .and_then(|start| stats.download_block_operations_end.map(|end| end - start));
        let load_data = stats
            .load_data_start
            .and_then(|start| stats.load_data_end.map(|end| end - start));
        let protocol_apply_block = stats
            .apply_block_start
            .and_then(|start| stats.apply_block_end.map(|end| end - start));
        let apply = stats
            .protocol_times
            .as_ref()
            .map(|p_times| p_times.apply_end - p_times.apply_start);
        let apply_begin_application = stats
            .protocol_times
            .as_ref()
            .map(|p_times| p_times.begin_application_end - p_times.begin_application_start);
        let apply_decoding_operations = stats
            .protocol_times
            .as_ref()
            .map(|p_times| p_times.operations_decoding_end - p_times.operations_decoding_start);
        let apply_encoding_operations_metadata = stats.protocol_times.as_ref().map(|p_times| {
            p_times.operations_metadata_encoding_end - p_times.operations_metadata_encoding_start
        });
        let apply_collecting_new_rolls = stats.protocol_times.as_ref().map(|p_times| {
            p_times.collect_new_rolls_owner_snapshots_end
                - p_times.collect_new_rolls_owner_snapshots_start
        });
        let apply_commit = stats
            .protocol_times
            .as_ref()
            .map(|p_times| p_times.commit_end - p_times.commit_start);
        let store_data = stats
            .store_result_start
            .and_then(|start| stats.store_result_end.map(|end| end - start));

        Self {
            precheck,
            send_data,
            download,
            download_block_header,
            download_block_operations,
            load_data,
            protocol_apply_block,
            apply,
            apply_begin_application,
            apply_decoding_operations,
            apply_encoding_operations_metadata,
            apply_collecting_new_rolls,
            apply_commit,
            store_data,
        }
    }
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
    pub received_time: Option<u64>,
    pub sent_time: Option<u64>,
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
            histogram
                .record(stats.received_time.unwrap_or_default())
                .unwrap()
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
