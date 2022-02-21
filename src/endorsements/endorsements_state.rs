use std::{collections::BTreeMap, str::FromStr};

use num::Zero;
use serde::Deserialize;
use strum_macros::EnumIter;
use time::{Duration, OffsetDateTime};
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Spans,
};

use crate::{
    extensions::{
        convert_time_to_unit_string, get_time_style, ExtendedTable, SortableByFocus, StyledTime,
        TuiTableData,
    },
    operations::OperationStats, baking::BlockApplicationStatistics,
};

pub type EndorsementRights = BTreeMap<String, Vec<u32>>;
pub type EndorsementStatuses = BTreeMap<String, EndorsementStatus>;
pub type EndorsementStatusSortableVec = Vec<EndorsementStatusSortable>;
pub type MempoolEndorsementStats = BTreeMap<String, OperationStats>;
pub type InjectedEndorsementStats = BTreeMap<i32, OperationStats>;

#[derive(Clone, Debug, Deserialize)]
pub struct EndorsementRightsWithTimePerLevel {
    pub level: i32,
    pub slots: Vec<u16>,
    pub delegate: String,
    #[serde(default)]
    #[serde(with = "time::serde::rfc3339::option")]
    pub estimated_time: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
pub struct EndorsementRightsWithTime {
    pub rights: BTreeMap<i32, Option<OffsetDateTime>>,
}

impl EndorsementRightsWithTime {
    pub fn new(raw: &[EndorsementRightsWithTimePerLevel]) -> Self {
        let organized = raw
            .iter()
            .map(|rights_per_level| {
                (
                    rights_per_level.level,
                    rights_per_level.estimated_time.clone(),
                )
            })
            .collect();

        Self { rights: organized }
    }

    // TODO: same thing as in baking rights, move to common trait?
    pub fn next_endorsing(
        &self,
        level: i32,
        block_timestamp: OffsetDateTime,
        block_delay: i32,
    ) -> Option<(i32, String)> {
        self.rights
            .range(level..)
            .next()
            .map(|(endorsement_level, time)| {
                if time.is_some() {
                    let now = OffsetDateTime::now_utc().unix_timestamp();
                    let block_time = block_timestamp.unix_timestamp();
                    let level_delta = endorsement_level - level;
                    let until_endorsing =
                        Duration::seconds(block_time + ((level_delta * block_delay) as i64) - now);
                    let mut final_str = String::from("");

                    if !until_endorsing.whole_days().is_zero() {
                        final_str += &format!("{} days", until_endorsing.whole_days());
                    } else if !until_endorsing.whole_hours().is_zero() {
                        final_str += &format!("{} hours", until_endorsing.whole_hours());
                    } else if !until_endorsing.whole_minutes().is_zero() {
                        final_str += &format!("{} minutes", until_endorsing.whole_minutes());
                    } else if !until_endorsing.whole_seconds().is_zero()
                        && until_endorsing.is_positive()
                    {
                        final_str += &format!("{} seconds", until_endorsing.whole_seconds());
                    } else {
                        final_str += &"now".to_string();
                    }
                    (*endorsement_level, final_str)
                } else {
                    (*endorsement_level, String::from(""))
                }
            })
    }
}

#[derive(Debug, Clone)]
pub struct EndrosementsState {
    pub endorsement_rights: EndorsementRights,
    pub endoresement_status_summary: BTreeMap<EndorsementState, usize>,
    pub endorsement_rights_with_time: EndorsementRightsWithTime,
    pub injected_endorsement_stats: InjectedEndorsementStats,
    pub last_endorsement_operation: Option<String>,
    pub last_injected_endorsement_summary: EndorsementOperationSummary,
    pub last_endrosement_operation_level: i32,

    // ui specific states
    pub endorsement_table: ExtendedTable<EndorsementStatusSortableVec>,
}

impl Default for EndrosementsState {
    fn default() -> Self {
        let endorsement_table = ExtendedTable::new(
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
                Constraint::Length(7),
                Constraint::Length(36),
                Constraint::Min(11),
                Constraint::Min(8),
                Constraint::Min(14),
                Constraint::Min(17),
                Constraint::Min(9),
                Constraint::Min(11),
                Constraint::Min(9),
                Constraint::Min(12),
            ],
            4,
        );

        Self {
            endorsement_table,
            endoresement_status_summary: BTreeMap::new(),
            endorsement_rights: BTreeMap::new(),
            endorsement_rights_with_time: Default::default(),
            last_endorsement_operation: None,
            injected_endorsement_stats: Default::default(),
            last_injected_endorsement_summary: Default::default(),
            last_endrosement_operation_level: Default::default(),
        }
    }
}

impl SortableByFocus for EndorsementStatusSortableVec {
    fn sort_by_focus(&mut self, focus_index: usize, delta_toggle: bool) {
        match focus_index {
            0 => self.sort_by_key(|k| k.slot_count),
            1 => self.sort_by_key(|k| k.baker.clone()),
            2 => self.sort_by_key(|k| k.state.clone()),
            3 => self.sort_by_key(|k| k.delta),
            4 => self.sort_by_key(|k| k.received_hash_time),
            5 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.received_contents_time_delta
                } else {
                    k.received_contents_time
                }
            }),
            6 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.decoded_time_delta
                } else {
                    k.decoded_time
                }
            }),
            7 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.prechecked_time_delta
                } else {
                    k.prechecked_time
                }
            }),
            8 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.applied_time_delta
                } else {
                    k.applied_time
                }
            }),
            9 => self.sort_by_key(|k| {
                if delta_toggle {
                    k.broadcast_time_delta
                } else {
                    k.broadcast_time
                }
            }),
            _ => {}
        }
    }

    fn rev(&mut self) {
        self.reverse()
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct EndorsementStatus {
    // pub block_timestamp: u64,
    pub decoded_time: Option<u64>,
    pub applied_time: Option<u64>,
    pub prechecked_time: Option<u64>,
    pub broadcast_time: Option<u64>,
    pub received_contents_time: Option<u64>,
    pub received_hash_time: Option<u64>,
    pub slot: u32,
    pub state: String,
    pub broadcast: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum EndorsementState {
    Missing = 0,
    Broadcast = 1,
    Applied = 2,
    Prechecked = 3,
    Decoded = 4,
    Received = 5,
}

#[derive(Clone, Debug, Default)]
pub struct EndorsementStatusSortable {
    pub delta: Option<u64>,
    pub decoded_time: Option<u64>,
    pub received_hash_time: Option<u64>,
    pub received_contents_time: Option<u64>,
    pub applied_time: Option<u64>,
    pub prechecked_time: Option<u64>,
    pub broadcast_time: Option<u64>,
    pub state: EndorsementState,

    // deltas
    pub received_contents_time_delta: Option<u64>,
    pub decoded_time_delta: Option<u64>,
    pub prechecked_time_delta: Option<u64>,
    pub applied_time_delta: Option<u64>,
    pub broadcast_time_delta: Option<u64>,

    pub baker: String,
    pub slot_count: usize,
}

impl EndorsementStatus {
    pub fn to_sortable(&self, baker: String, slot_count: usize) -> EndorsementStatusSortable {
        let delta = if let (Some(broadcast), Some(received)) =
            (self.broadcast_time, self.received_hash_time)
        {
            Some(broadcast - received)
        } else {
            None
        };

        let received_contents_time_delta =
            if let (Some(received_contents_time), Some(received_hash_time)) =
                (self.received_contents_time, self.received_hash_time)
            {
                Some(received_contents_time - received_hash_time)
            } else {
                None
            };

        let decoded_time_delta = if let (Some(decoded_time), Some(received_contents_time)) =
            (self.decoded_time, self.received_contents_time)
        {
            Some(decoded_time - received_contents_time)
        } else {
            None
        };

        let prechecked_time_delta = if let (Some(prechecked_time), Some(decoded_time)) =
            (self.prechecked_time, self.decoded_time)
        {
            Some(prechecked_time - decoded_time)
        } else {
            None
        };

        let applied_time_delta = if let (Some(applied_time), Some(decoded_time)) =
            (self.applied_time, self.decoded_time)
        {
            Some(applied_time - decoded_time)
        } else {
            None
        };

        let broadcast_time_delta = if let (Some(broadcast_time), Some(applied_time)) =
            (self.broadcast_time, self.applied_time)
        {
            Some(broadcast_time - applied_time)
        } else if let (Some(broadcast_time), Some(prechecked_time)) =
            (self.broadcast_time, self.prechecked_time)
        {
            Some(broadcast_time - prechecked_time)
        } else {
            None
        };

        EndorsementStatusSortable {
            baker,
            slot_count,
            delta,
            received_hash_time: self.received_hash_time,
            received_contents_time: self.received_contents_time,
            decoded_time: self.decoded_time,
            prechecked_time: self.prechecked_time,
            applied_time: self.applied_time,
            broadcast_time: self.broadcast_time,
            state: EndorsementState::from_str(&self.state).unwrap_or_default(),
            received_contents_time_delta,
            decoded_time_delta,
            prechecked_time_delta,
            applied_time_delta,
            broadcast_time_delta,
        }
    }
}

impl EndorsementStatusSortable {
    pub fn new(baker: String, slot_count: usize) -> Self {
        Self {
            baker,
            slot_count,
            ..Default::default()
        }
    }
}

impl TuiTableData for EndorsementStatusSortable {
    fn construct_tui_table_data(&self, delta_toggle: bool) -> Vec<(String, Style)> {
        let mut final_vec = Vec::with_capacity(9);
        let missing_value = (String::from('-'), Style::default().fg(Color::DarkGray));

        final_vec.push((
            self.slot_count.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM),
        ));
        final_vec.push((
            self.baker.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM),
        ));
        final_vec.push((self.state.to_string(), self.state.get_style()));

        if let Some(delta) = self.delta {
            final_vec.push((convert_time_to_unit_string(delta), get_time_style(delta)))
        } else {
            final_vec.push(missing_value.clone());
        }

        if let Some(received_hash_time) = self.received_hash_time {
            final_vec.push((
                convert_time_to_unit_string(received_hash_time),
                get_time_style(received_hash_time),
            ));
        } else {
            final_vec.push(missing_value.clone());
        }

        if delta_toggle {
            if let Some(received_contents_time_delta) = self.received_contents_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(received_contents_time_delta),
                    get_time_style(received_contents_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(decoded_time_delta) = self.decoded_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(decoded_time_delta),
                    get_time_style(decoded_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(prechecked_time_delta) = self.prechecked_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(prechecked_time_delta),
                    get_time_style(prechecked_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(applied_time_delta) = self.applied_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(applied_time_delta),
                    get_time_style(applied_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(broadcast_time_delta) = self.broadcast_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(broadcast_time_delta),
                    get_time_style(broadcast_time_delta),
                ));
            } else {
                final_vec.push(missing_value);
            }
        } else {
            if let Some(received_contents_time) = self.received_contents_time {
                final_vec.push((
                    convert_time_to_unit_string(received_contents_time),
                    get_time_style(received_contents_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(decoded_time) = self.decoded_time {
                final_vec.push((
                    convert_time_to_unit_string(decoded_time),
                    get_time_style(decoded_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(prechecked_time) = self.prechecked_time {
                final_vec.push((
                    convert_time_to_unit_string(prechecked_time),
                    get_time_style(prechecked_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(applied_time) = self.applied_time {
                final_vec.push((
                    convert_time_to_unit_string(applied_time),
                    get_time_style(applied_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(broadcast_time) = self.broadcast_time {
                final_vec.push((
                    convert_time_to_unit_string(broadcast_time),
                    get_time_style(broadcast_time),
                ));
            } else {
                final_vec.push(missing_value);
            }
        }

        final_vec
    }
}

pub struct InvalidVariantError {}

impl FromStr for EndorsementState {
    type Err = InvalidVariantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "missing" => Ok(EndorsementState::Missing),
            "broadcast" => Ok(EndorsementState::Broadcast),
            "applied" => Ok(EndorsementState::Applied),
            "prechecked" => Ok(EndorsementState::Prechecked),
            "decoded" => Ok(EndorsementState::Decoded),
            "received" => Ok(EndorsementState::Received),
            _ => Err(InvalidVariantError {}),
        }
    }
}

impl ToString for EndorsementState {
    fn to_string(&self) -> String {
        match self {
            EndorsementState::Missing => String::from("Missing"),
            EndorsementState::Broadcast => String::from("Broadcast"),
            EndorsementState::Applied => String::from("Applied"),
            EndorsementState::Prechecked => String::from("Prechecked"),
            EndorsementState::Decoded => String::from("Decoded"),
            EndorsementState::Received => String::from("Received"),
        }
    }
}

impl Default for EndorsementState {
    fn default() -> Self {
        Self::Missing
    }
}

impl EndorsementState {
    pub fn get_style(&self) -> Style {
        let style: Style = Style::default();
        match self {
            EndorsementState::Missing => style.bg(Color::Red),
            EndorsementState::Broadcast => style.bg(Color::Green),
            EndorsementState::Applied => style.bg(Color::Cyan),
            EndorsementState::Prechecked => style.bg(Color::Blue),
            EndorsementState::Decoded => style.bg(Color::Magenta),
            EndorsementState::Received => style.bg(Color::Yellow),
        }
    }

    pub fn get_style_fg(&self) -> Style {
        let style: Style = Style::default();
        match self {
            EndorsementState::Missing => style.fg(Color::Red),
            EndorsementState::Broadcast => style.fg(Color::Green),
            EndorsementState::Applied => style.fg(Color::Cyan),
            EndorsementState::Prechecked => style.fg(Color::Blue),
            EndorsementState::Decoded => style.fg(Color::Magenta),
            EndorsementState::Received => style.fg(Color::Yellow),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct EndorsementOperationSummary {
    pub block_application: Option<i128>,
    pub block_received: Option<i128>,
    pub injected: Option<i128>,
    pub validated: Option<i128>,
    pub operation_hash_sent: Option<i128>,
    pub operation_requested: Option<i128>,
    pub operation_sent: Option<i128>,
    pub operation_hash_received_back: Option<u64>,
}

impl EndorsementOperationSummary {
    pub fn new(current_head_timestamp: OffsetDateTime, op_stats: OperationStats, block_stats: Option<BlockApplicationStatistics>) -> Self {
        let block_received = block_stats.clone().map(|stats| {
            let current_head_nanos = current_head_timestamp.unix_timestamp_nanos();
            (stats.receive_timestamp as i128) - current_head_nanos
        });

        let block_application = block_stats.clone().and_then(|stats| {
            stats.apply_block_end.and_then(|end| stats.apply_block_start.map(|start| (end - start) as i128))
        });

        let injected = op_stats.injected_timestamp.and_then(|inject_time| {
                block_stats.map(|stats| (inject_time as i128) - stats.receive_timestamp)
        });

        let validated = op_stats.validation_duration();

        let operation_hash_sent = op_stats
            .first_sent()
            .and_then(|sent| op_stats.validation_ended().map(|v_end| sent - v_end));

        let operation_requested = op_stats
            .first_content_requested_remote()
            .and_then(|op_req| op_stats.first_sent().map(|sent| op_req - sent));

        let operation_sent = op_stats.first_content_sent().and_then(|cont_sent| {
            op_stats
                .first_content_requested_remote()
                .map(|op_req| cont_sent - op_req)
        });

        Self {
            block_received,
            block_application,
            injected,
            validated,
            operation_hash_sent,
            operation_requested,
            operation_sent,
            operation_hash_received_back: None, // TODO
        }
    }

    pub fn to_table_data(&self) -> Vec<(Spans, StyledTime<i128>)> {
        vec![
            (Spans::from("Block Received"), StyledTime::new(self.block_received)),
            (Spans::from("Block Application"), StyledTime::new(self.block_application)),
            (Spans::from("Endorsement Operation Injected"), StyledTime::new(self.injected)),
            (Spans::from("Endorsement Operation Validated"), StyledTime::new(self.validated)),
            (
                Spans::from("Endorsement Operation Hash Sent"),
                StyledTime::new(self.operation_hash_sent),
            ),
            (
                Spans::from("Endorsement Operation Requested"),
                StyledTime::new(self.operation_requested),
            ),
            (
                Spans::from("Endorsement Operation Sent"),
                StyledTime::new(self.operation_sent),
            ),
            // (
            //     Spans::from("Operation Hash Received back"),
            //     StyledTime::new(None),
            // ),
        ]
    }
}
