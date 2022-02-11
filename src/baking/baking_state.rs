use hdrhistogram::Histogram;

use serde::Deserialize;
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};

use crate::extensions::{
    convert_time_to_unit_string, convert_time_to_unit_string_option, ExtendedTable,
    SortableByFocus, TuiTableData,
};

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
                Spans::from("2 Download"),
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
                Spans::from("3 Load Data"),
                convert_time_to_unit_string_option(self.load_data),
            ),
            (
                Spans::from("4 Protocol Apply Block"),
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
                Spans::from("5 Store data"),
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
    pub sent_end_time: Option<u64>,
    pub sent_start_time: Option<u64>,
    pub get_operations_recv_end_time: Option<u64>,
    pub get_operations_recv_start_time: Option<u64>,
    pub operations_send_start_time: Option<u64>,
    pub operations_send_end_time: Option<u64>,
}

impl SortableByFocus for PerPeerBlockStatisticsVector {
    fn sort_by_focus(&mut self, focus_index: usize, delta_toogle: bool) {
        match focus_index {
            0 => self.sort_by_key(|s| s.address.clone()),
            1 => self.sort_by_key(|s| s.node_id.clone()),
            2 => self.sort_by_key(|s| s.received_time),
            3 => self.sort_by_key(|s| s.sent_time),
            // TODO: fix this with delta toggle
            4 => self.sort_by_key(|s| s.get_operations_recv_end_time),
            5 => self.sort_by_key(|s| s.operations_send_end_time),
            _ => {}
        }
    }

    fn rev(&mut self) {
        self.reverse()
    }
}

impl TuiTableData for PerPeerBlockStatistics {
    fn construct_tui_table_data(&self, _: bool) -> Vec<(String, Style)> {
        let style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);

        vec![
            (self.address.clone(), style),
            (self.node_id.clone(), style),
            (
                convert_time_to_unit_string_option(self.received_time),
                style,
            ),
            (
                convert_time_to_unit_string_option(self.sent_end_time),
                style,
            ),
            (
                convert_time_to_unit_string_option(self.get_operations_recv_end_time),
                style,
            ),
            (
                convert_time_to_unit_string_option(self.operations_send_end_time),
                style,
            ),
        ]
    }
}

pub struct BakingSummary {
    pub injected: Option<u64>,
    pub send_block_header: Option<u64>,
    pub block_operations_requested: Option<u64>,
    pub block_operations_sent: Option<u64>,
    pub block_header_received_back: Option<u64>,
}

impl BakingSummary {
    pub fn extend_table_data(&self, table_data: &mut Vec<(Spans, String)>) {
        let injected = (
            Spans::from("1 Injected"),
            convert_time_to_unit_string_option(self.injected),
        );
        let send_block_header = (
            Spans::from("6 Send Block Header"),
            convert_time_to_unit_string_option(self.send_block_header),
        );
        let block_operations_requested = (
            Spans::from("7 Block Operations Requested"),
            convert_time_to_unit_string_option(self.block_operations_requested),
        );
        let block_operations_sent = (
            Spans::from("8 Block Operations Sent"),
            convert_time_to_unit_string_option(self.block_operations_sent),
        );
        let block_header_received_back = (
            Spans::from("9 Block Header Reveived Back"),
            convert_time_to_unit_string_option(self.block_header_received_back),
        );

        table_data.insert(0, injected);
        table_data.push(send_block_header);
        table_data.push(block_operations_requested);
        table_data.push(block_operations_sent);
        table_data.push(block_header_received_back);
    }
}

impl From<PerPeerBlockStatisticsVector> for BakingSummary {
    fn from(stats: PerPeerBlockStatisticsVector) -> Self {
        let injected = stats
            .iter()
            .find(|stat| stat.received_time == Some(0))
            .map(|stat| stat.received_time.unwrap_or(0));
        let send_block_header = stats.iter().filter_map(|stat| stat.sent_end_time).min();
        let block_operations_requested = stats
            .iter()
            .filter_map(|stat| stat.get_operations_recv_end_time)
            .min();
        let block_operations_sent = stats
            .iter()
            .filter_map(|stat| stat.operations_send_end_time)
            .min();
        let block_header_received_back = stats
            .iter()
            .filter_map(|stat| stat.received_time)
            .filter(|received| received != &0)
            .min();

        Self {
            injected,
            send_block_header,
            block_operations_requested,
            block_operations_sent,
            block_header_received_back,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BakingState {
    pub application_statistics: Vec<BlockApplicationStatistics>,
    pub per_peer_block_statistics: PerPeerBlockStatisticsVector,
    pub baking_table: ExtendedTable<PerPeerBlockStatisticsVector>,
}

impl Default for BakingState {
    fn default() -> Self {
        let baking_table = ExtendedTable::new(
            vec![
                "Address",
                "Node Id",
                "Header Received",
                "Header Sent",
                "OP Requested",
                "OP Sent",
            ]
            .iter()
            .map(|v| v.to_string())
            .collect(),
            vec![
                Constraint::Length(22), // 003.228.018.204:9732
                Constraint::Length(30), // idsvin1UKjua9Fppj3oDZePNkBPaWT
                Constraint::Min(17),
                Constraint::Min(13),
                Constraint::Min(14),
                Constraint::Min(9),
            ],
            4,
        );

        Self {
            baking_table,
            application_statistics: Default::default(),
            per_peer_block_statistics: Default::default(),
        }
    }
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
