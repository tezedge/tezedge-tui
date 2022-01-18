use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use strum_macros::{Display, ToString};
use tui::style::Color;

use super::SortableByFocus;

// use super::convert_time_to_unit_string;

pub type OperationsStats = BTreeMap<String, OperationStats>;
pub type OperationsStatsSortable = Vec<OperationStatsSortable>;

#[derive(Deserialize, Clone, Debug)]
pub struct OperationStats {
    kind: Option<OperationKind>,
    /// Minimum time when we saw this operation. Latencies are measured
    /// from this point.
    min_time: Option<u64>,
    first_block_timestamp: Option<u64>,
    validation_started: Option<i128>,
    /// (time_validation_finished, validation_result, prevalidation_duration)
    validation_result: Option<(i128, OperationValidationResult, Option<i128>, Option<i128>)>,
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

#[derive(Deserialize, Debug, Clone, Copy, Display, PartialEq, Eq, PartialOrd, Ord)]
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
    Prechecked,
    PrecheckRefused,
    Prevalidate,
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
    pub datetime: u64,
    pub hash: String,
    pub nodes: usize,
    pub delta: Option<i128>,
    pub received: Option<i128>,
    pub content_received: Option<i128>,
    pub validation_started: Option<i128>,
    pub preapply_started: Option<i128>,
    pub preapply_ended: Option<i128>,
    pub validation_finished: Option<i128>,
    pub validations_length: usize,
    pub sent: Option<i128>,
    pub kind: Option<OperationKind>,

    // Deltas
    pub content_received_delta: Option<i128>,
    pub validation_started_delta: Option<i128>,

    pub preapply_started_delta: Option<i128>,
    pub preapply_ended_delta: Option<i128>,
    pub validation_finished_delta: Option<i128>,
    pub sent_delta: Option<i128>,
}

impl OperationStats {
    pub fn to_statistics_sortable(&self, hash: String) -> OperationStatsSortable {
        let nodes = self.nodes.len();
        let first_received = self
            .nodes
            .clone()
            .into_iter()
            .filter_map(|(_, v)| {
                v.received
                    .into_iter()
                    .min_by_key(|v| v.latency)
                    .map(|v| v.latency)
            })
            .min();

        let first_sent = self
            .nodes
            .clone()
            .into_iter()
            .filter_map(|(_, v)| {
                v.sent
                    .into_iter()
                    .min_by_key(|v| v.latency)
                    .map(|v| v.latency)
            })
            .min();

        let delta = if let (Some(first_received), Some(first_sent)) = (first_received, first_sent) {
            Some(first_sent - first_received)
        } else {
            None
        };

        let content_received = self
            .nodes
            .clone()
            .into_iter()
            .filter_map(|(_, v)| v.content_received.into_iter().min())
            .min();

        let (validation_finished, _, preapply_started, preapply_ended) =
            if let Some(validation_res) = self.validation_result {
                (
                    Some(validation_res.0),
                    Some(validation_res.1),
                    validation_res.2,
                    validation_res.3,
                )
            } else {
                (None, None, None, None)
            };

        let validations_length = self.validations.len();

        // Deltas
        let content_received_delta = if let (Some(content_received), Some(first_received)) =
            (content_received, first_received)
        {
            Some(content_received - first_received)
        } else {
            None
        };

        let validation_started_delta = if let (Some(validation_started), Some(content_received)) =
            (self.validation_started, content_received)
        {
            Some(validation_started - content_received)
        } else {
            None
        };

        let preapply_started_delta = if let (Some(preapply_started), Some(validation_started)) =
            (preapply_started, self.validation_started)
        {
            Some(preapply_started - validation_started)
        } else {
            None
        };

        let preapply_ended_delta = if let (Some(preapply_started), Some(preapply_ended)) =
            (preapply_started, preapply_ended)
        {
            Some(preapply_ended - preapply_started)
        } else {
            None
        };

        let validation_finished_delta =
            if let (Some(validation_started), Some(validation_finished)) =
                (self.validation_started, validation_finished)
            {
                Some(validation_finished - validation_started)
            } else {
                None
            };

        let sent_delta = if let (Some(first_sent), Some(validation_finished)) =
            (first_sent, validation_finished)
        {
            Some(first_sent - validation_finished)
        } else {
            None
        };

        OperationStatsSortable {
            datetime: self.first_block_timestamp.unwrap_or_default(),
            hash,
            nodes,
            delta,
            received: first_received,
            content_received,
            validation_started: self.validation_started,
            preapply_started,
            preapply_ended,
            validation_finished,
            validations_length,
            sent: first_sent,
            kind: self.kind,
            content_received_delta,
            validation_started_delta,
            preapply_started_delta,
            preapply_ended_delta,
            validation_finished_delta,
            sent_delta,
        }
    }

    pub fn to_operations_details(&self) -> Vec<OperationDetailSortable> {
        self.nodes
            .iter()
            .map(|(node_id, stats)| {
                let first_received = stats.received.clone().into_iter().next().map(|v| v.latency);
                let first_content_received = stats.content_received.clone().into_iter().next();
                let first_sent = stats
                    .clone()
                    .sent
                    .into_iter()
                    .min_by_key(|v| v.latency)
                    .map(|v| v.latency);
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

pub trait TuiTableData {
    fn construct_tui_table_data(&self, delta_toggle: bool) -> Vec<(String, Color)>;
}

impl TuiTableData for OperationStatsSortable {
    fn construct_tui_table_data(&self, delta_toggle: bool) -> Vec<(String, Color)> {
        let mut final_vec = Vec::with_capacity(13);

        let datetime =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.datetime as i64, 0), Utc)
                .format("%H:%M:%S, %Y-%m-%d");

        final_vec.push((datetime.to_string(), Color::Gray));
        final_vec.push((self.hash.clone(), Color::Reset));
        final_vec.push((self.nodes.to_string(), Color::Gray));

        if let Some(delta) = self.delta {
            final_vec.push((convert_time_to_unit_string(delta), get_color(delta)));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
        }

        if let Some(received) = self.received {
            final_vec.push((convert_time_to_unit_string(received), Color::Reset));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
        }

        // Diferent output based on a toggle

        if delta_toggle {
            if let Some(content_received_delta) = self.content_received_delta {
                final_vec.push((
                    convert_time_to_unit_string(content_received_delta),
                    get_color(content_received_delta),
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(validation_started_delta) = self.validation_started_delta {
                final_vec.push((
                    convert_time_to_unit_string(validation_started_delta),
                    get_color(validation_started_delta),
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(preapply_started_delta) = self.preapply_started_delta {
                final_vec.push((
                    convert_time_to_unit_string(preapply_started_delta),
                    get_color(preapply_started_delta),
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(preapply_ended_delta) = self.preapply_ended_delta {
                final_vec.push((
                    convert_time_to_unit_string(preapply_ended_delta),
                    get_color(preapply_ended_delta),
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(validation_finished_delta) = self.validation_finished_delta {
                final_vec.push((
                    convert_time_to_unit_string(validation_finished_delta),
                    get_color(validation_finished_delta),
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }
        } else {
            if let Some(content_received) = self.content_received {
                final_vec.push((convert_time_to_unit_string(content_received), Color::Reset));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(validation_started) = self.validation_started {
                final_vec.push((
                    convert_time_to_unit_string(validation_started),
                    Color::Reset,
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(preapply_started) = self.preapply_started {
                final_vec.push((convert_time_to_unit_string(preapply_started), Color::Reset));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(preapply_ended) = self.preapply_ended {
                final_vec.push((convert_time_to_unit_string(preapply_ended), Color::Reset));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }

            if let Some(validation_finished) = self.validation_finished {
                final_vec.push((
                    convert_time_to_unit_string(validation_finished),
                    Color::Reset,
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }
        }

        if self.validations_length != 0 {
            final_vec.push((self.validations_length.to_string(), Color::Gray));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
        }

        if delta_toggle {
            if let Some(sent_delta) = self.sent_delta {
                final_vec.push((
                    convert_time_to_unit_string(sent_delta),
                    get_color(sent_delta),
                ));
            } else {
                final_vec.push((String::from('-'), Color::DarkGray));
            }
        } else if let Some(sent) = self.sent {
            final_vec.push((convert_time_to_unit_string(sent), Color::Reset));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
        }

        if let Some(kind) = self.kind {
            final_vec.push((kind.to_string(), Color::Reset));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
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
    pub first_received: Option<i128>,
    pub first_content_received: Option<i128>,
    pub first_sent: Option<i128>,
    pub received: usize,
    pub content_received: usize,
    pub sent: usize,
}

impl TuiTableData for OperationDetailSortable {
    fn construct_tui_table_data(&self, delta_toggle: bool) -> Vec<(String, Color)> {
        let mut final_vec = Vec::with_capacity(7);

        final_vec.push((self.node_id.clone(), Color::Reset));

        if let Some(first_received) = self.first_received {
            final_vec.push((convert_time_to_unit_string(first_received), Color::Reset));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
        }

        if let Some(first_content_received) = self.first_content_received {
            final_vec.push((
                convert_time_to_unit_string(first_content_received),
                Color::Reset,
            ));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
        }

        if let Some(first_sent) = self.first_sent {
            final_vec.push((convert_time_to_unit_string(first_sent), Color::Reset));
        } else {
            final_vec.push((String::from('-'), Color::DarkGray));
        }

        final_vec.push((self.received.to_string(), Color::Reset));
        final_vec.push((self.content_received.to_string(), Color::Reset));
        final_vec.push((self.sent.to_string(), Color::Reset));

        final_vec
    }
}

fn get_color(value: i128) -> Color {
    if value < 20000000 {
        Color::Reset
    } else if value < 50000000 {
        Color::Rgb(255, 165, 0) // orange
    } else {
        Color::Red
    }
}

impl SortableByFocus for OperationsStatsSortable {
    fn sort_by_focus(&mut self, focus_index: usize, delta_toggle: bool) {
        match focus_index {
            0 => self.sort_by_key(|k| k.datetime),
            1 => self.sort_by_key(|k| k.hash.clone()),
            2 => self.sort_by_key(|k| k.nodes),
            3 => self.sort_by_key(|k| k.delta),
            4 => self.sort_by_key(|k| k.received),
            5 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.content_received_delta
                } else {
                    k.content_received
                }
            }),
            6 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.validation_started_delta
                } else {
                    k.validation_started
                }
            }),
            7 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.preapply_started_delta
                } else {
                    k.preapply_started
                }
            }),
            8 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.preapply_ended_delta
                } else {
                    k.preapply_ended
                }
            }),
            9 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.validation_finished_delta
                } else {
                    k.validation_finished
                }
            }),
            10 => self.sort_by_key(|k| k.validations_length),
            11 => self.sort_by_key(|k| if delta_toggle { k.sent_delta } else { k.sent }),
            12 => self.sort_by_key(|k| k.kind),
            _ => {}
        }
    }
}
