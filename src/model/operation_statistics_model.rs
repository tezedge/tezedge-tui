use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use strum_macros::{Display, ToString};

// use super::convert_time_to_unit_string;

pub type OperationsStats = BTreeMap<String, OperationStats>;
pub type OperationsStatsSortable = Vec<OperationStatsSortable>;

#[derive(Deserialize, Clone, Debug)]
pub struct OperationStats {
    kind: Option<OperationKind>,
    /// Minimum time when we saw this operation. Latencies are measured
    /// from this point.
    min_time: Option<i128>,
    first_block_timestamp: Option<i128>,
    validation_started: Option<i128>,
    validation_result: Option<(i128, OperationValidationResult, i128, i128)>,
    validations: Vec<OperationValidationStats>,
    nodes: HashMap<String, OperationNodeStats>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OperationNodeStats {
    received: Vec<OperationNodeCurrentHeadStats>,
    sent: Vec<OperationNodeCurrentHeadStats>,

    content_requested: Vec<i128>,
    content_received: Vec<i128>,

    content_requested_remote: Vec<i128>,
    content_sent: Vec<i128>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct OperationNodeCurrentHeadStats {
    /// Latency from first time we have seen that operation.
    latency: i128,
    block_level: i32,
    block_timestamp: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OperationValidationStats {
    started: Option<i128>,
    finished: Option<i128>,
    preapply_started: Option<i128>,
    preapply_ended: Option<i128>,
    current_head_level: Option<i32>,
    result: Option<OperationValidationResult>,
}

#[derive(Deserialize, Debug, Clone, Copy, Display)]
pub enum OperationKind {
    Endorsement,
    SeedNonceRevelation,
    DoubleEndorsement,
    DoubleBaking,
    Activation,
    Proposals,
    Ballot,
    EndorsementWithSlot,
    FailingNoop,
    Reveal,
    Transaction,
    Origination,
    Delegation,
    RegisterConstant,
    Unknown,
    Default,
}

impl Default for OperationKind {
    fn default() -> Self {
        OperationKind::Default
    }
}

#[derive(Deserialize, Debug, Clone, Copy, Display)]
pub enum OperationValidationResult {
    Applied,
    Refused,
    BranchRefused,
    BranchDelayed,
    Default,
}

// impl ToString for OperationValidationResult {
//     fn to_string(&self) -> String {
//         match self {
//             OperationValidationResult::Applied => String::from("Applied"),
//             OperationValidationResult::Refused => String::from("Refused"),
//             OperationValidationResult::BranchRefused => String::from("BranchRefused"),
//             OperationValidationResult::BranchDelayed => String::from("BranchDelayed"),
//             OperationValidationResult::Default => String::from("-"),
//         }
//     }
// }

impl Default for OperationValidationResult {
    fn default() -> Self {
        OperationValidationResult::Default
    }
}

#[derive(Clone, Debug)]
pub struct OperationStatsSortable {
    pub datetime: String,
    pub hash: String,
    pub nodes: usize,
    pub delta: i128,
    pub received: i128,
    pub content_received: i128,
    pub validation_started: i128,
    pub preapply_started: i128,
    pub preapply_ended: i128,
    pub validation_finished: i128,
    pub validations_length: usize,
    pub sent: i128,
    pub kind: OperationKind,
}

impl OperationStats {
    pub fn to_statistics_sortable(&self, hash: String) -> OperationStatsSortable {
        let nodes = self.nodes.len();
        let first_received = self
            .nodes
            .clone()
            .into_iter()
            .map(|(_, v)| {
                v.received
                    .into_iter()
                    .min_by_key(|v| v.latency)
                    .map(|v| v.latency)
                    .unwrap_or_default()
            })
            .min()
            .unwrap_or_default();

        let first_sent = self
            .nodes
            .clone()
            .into_iter()
            .map(|(_, v)| {
                v.sent
                    .into_iter()
                    .min_by_key(|v| v.latency)
                    .map(|v| v.latency)
                    .unwrap_or_default()
            })
            .min()
            .unwrap_or_default();

        let delta = if first_received == 0 || first_sent == 0 {
            0
        } else {
            first_sent - first_received
        };

        let content_received = self
            .nodes
            .clone()
            .into_iter()
            .map(|(_, v)| v.content_received.into_iter().min().unwrap_or_default())
            .min()
            .unwrap_or_default();

        let validation_started = self.validation_started.unwrap_or_default();
        let (validation_finished, _, preapply_started, preapply_ended) =
            self.validation_result.unwrap_or_default();

        let validations_length = self.validations.len();
        let kind = self.kind.unwrap_or_default();

        let datetime = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(self.first_block_timestamp.unwrap_or_default() as i64, 0),
            Utc,
        )
        .format("%H:%M:%S, %Y-%m-%d");

        OperationStatsSortable {
            datetime: datetime.to_string(), // TODO
            hash,
            nodes,
            delta,
            received: first_received,
            content_received,
            validation_started,
            preapply_started,
            preapply_ended,
            validation_finished,
            validations_length,
            sent: first_sent,
            kind,
        }
    }

    pub fn to_operations_details(&self) -> Vec<OperationDetailSortable> {
        self.nodes
            .iter()
            .map(|(node_id, stats)| {
                let first_received = stats
                    .received
                    .clone()
                    .into_iter()
                    .next()
                    .unwrap_or_default()
                    .latency;
                let first_content_received = stats
                    .content_received
                    .clone()
                    .into_iter()
                    .next()
                    .unwrap_or_default();
                let first_sent = stats
                    .content_sent
                    .clone()
                    .into_iter()
                    .next()
                    .unwrap_or_default();
                let received = stats.received.len();
                let content_received = stats.content_received.len();
                let sent = stats.sent.len();

                OperationDetailSortable {
                    node_id: node_id.clone(),
                    first_received,
                    first_content_received,
                    first_sent,
                    received,
                    content_received,
                    sent,
                }
            })
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl OperationStatsSortable {
    pub fn construct_tui_table_data(&self) -> Vec<String> {
        let mut final_vec = Vec::with_capacity(13);

        final_vec.push(self.datetime.to_string());
        final_vec.push(self.hash.clone());
        final_vec.push(self.nodes.to_string());

        if self.delta != 0 {
            final_vec.push(convert_time_to_unit_string(self.delta));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.received != 0 {
            final_vec.push(convert_time_to_unit_string(self.received));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.content_received != 0 {
            final_vec.push(convert_time_to_unit_string(self.content_received));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.validation_started != 0 {
            final_vec.push(convert_time_to_unit_string(self.validation_started));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.preapply_started != 0 {
            final_vec.push(convert_time_to_unit_string(self.preapply_started));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.preapply_ended != 0 {
            final_vec.push(convert_time_to_unit_string(self.preapply_ended));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.validation_finished != 0 {
            final_vec.push(convert_time_to_unit_string(self.validation_finished));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.validations_length != 0 {
            final_vec.push(self.validations_length.to_string());
        } else {
            final_vec.push(String::from('-'));
        }

        if self.sent != 0 {
            final_vec.push(convert_time_to_unit_string(self.sent));
        } else {
            final_vec.push(String::from('-'));
        }

        if let OperationKind::Default = self.kind {
            final_vec.push(String::from('-'));
        } else {
            final_vec.push(self.kind.to_string());
        }

        final_vec
    }
}

// TODO: fix this duplicate fn
fn convert_time_to_unit_string(time: i128) -> String {
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

pub struct OperationDetailSortable {
    pub node_id: String,
    pub first_received: i128,
    pub first_content_received: i128,
    pub first_sent: i128,
    pub received: usize,
    pub content_received: usize,
    pub sent: usize,
}

impl OperationDetailSortable {
    pub fn construct_tui_table_data(&self) -> Vec<String> {
        let mut final_vec = Vec::with_capacity(7);

        final_vec.push(self.node_id.clone());

        if self.first_received != 0 {
            final_vec.push(convert_time_to_unit_string(self.first_received));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.first_content_received != 0 {
            final_vec.push(convert_time_to_unit_string(self.first_content_received));
        } else {
            final_vec.push(String::from('-'));
        }

        if self.first_sent != 0 {
            final_vec.push(convert_time_to_unit_string(self.first_sent));
        } else {
            final_vec.push(String::from('-'));
        }

        final_vec.push(self.received.to_string());
        final_vec.push(self.content_received.to_string());
        final_vec.push(self.sent.to_string());

        final_vec
    }
}
