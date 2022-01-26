use std::{collections::BTreeMap, str::FromStr};

use serde::Deserialize;
use tui::{layout::Constraint, style::Color};

use crate::extensions::{
    convert_time_to_unit_string, get_color, ExtendedTable, SortableByFocus, TuiTableData,
};

pub type EndorsementRights = BTreeMap<String, Vec<u32>>;
pub type EndorsementStatuses = BTreeMap<String, EndorsementStatus>;
pub type EndorsementStatusSortableVec = Vec<EndorsementStatusSortable>;

#[derive(Debug, Clone)]
pub struct EndrosementsState {
    pub endorsement_rights: EndorsementRights,
    pub current_head_endorsement_statuses: EndorsementStatusSortableVec,
    pub endoresement_status_summary: BTreeMap<EndorsementState, usize>,

    // ui specific states
    pub endorsement_table: ExtendedTable,
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
                Constraint::Length(6),
                Constraint::Length(36),
                Constraint::Min(11),
                Constraint::Min(8),
                Constraint::Min(12),
                Constraint::Min(15),
                Constraint::Min(8),
                Constraint::Min(9),
                Constraint::Min(8),
                Constraint::Min(10),
            ],
            4,
        );

        Self {
            endorsement_table,
            current_head_endorsement_statuses: Vec::new(),
            endoresement_status_summary: BTreeMap::new(),
            endorsement_rights: BTreeMap::new(),
        }
    }
}

impl SortableByFocus for EndorsementStatusSortableVec {
    fn sort_by_focus(&mut self, focus_index: usize, _delta_toggle: bool) {
        match focus_index {
            0 => self.sort_by_key(|k| k.slot_count),
            1 => self.sort_by_key(|k| k.baker.clone()),
            2 => self.sort_by_key(|k| k.state.clone()),
            3 => self.sort_by_key(|k| k.delta),
            4 => self.sort_by_key(|k| k.received_hash_time),
            5 => self.sort_by_key(|k| k.received_contents_time),
            6 => self.sort_by_key(|k| k.decoded_time),
            7 => self.sort_by_key(|k| k.prechecked_time),
            8 => self.sort_by_key(|k| k.applied_time),
            9 => self.sort_by_key(|k| k.broadcast_time),
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    // TODO: fix sorting, use Options
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
    fn construct_tui_table_data(&self, delta_toggle: bool) -> Vec<(String, Color)> {
        let mut final_vec = Vec::with_capacity(9);
        let missing_value = (String::from('-'), Color::DarkGray);

        final_vec.push((self.slot_count.to_string(), Color::White));
        final_vec.push((self.baker.clone(), Color::White));
        final_vec.push((self.state.to_string(), Color::Reset));

        if let Some(delta) = self.delta {
            final_vec.push((convert_time_to_unit_string(delta), get_color(delta)))
        } else {
            final_vec.push(missing_value.clone());
        }

        if let Some(received_hash_time) = self.received_hash_time {
            final_vec.push((
                convert_time_to_unit_string(received_hash_time),
                get_color(received_hash_time),
            ));
        } else {
            final_vec.push(missing_value.clone());
        }

        if delta_toggle {
            if let Some(received_contents_time_delta) = self.received_contents_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(received_contents_time_delta),
                    get_color(received_contents_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(decoded_time_delta) = self.decoded_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(decoded_time_delta),
                    get_color(decoded_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(prechecked_time_delta) = self.prechecked_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(prechecked_time_delta),
                    get_color(prechecked_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(applied_time_delta) = self.applied_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(applied_time_delta),
                    get_color(applied_time_delta),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(broadcast_time_delta) = self.broadcast_time_delta {
                final_vec.push((
                    convert_time_to_unit_string(broadcast_time_delta),
                    get_color(broadcast_time_delta),
                ));
            } else {
                final_vec.push(missing_value);
            }
        } else {
            if let Some(received_contents_time) = self.received_contents_time {
                final_vec.push((
                    convert_time_to_unit_string(received_contents_time),
                    get_color(received_contents_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(decoded_time) = self.decoded_time {
                final_vec.push((
                    convert_time_to_unit_string(decoded_time),
                    get_color(decoded_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(prechecked_time) = self.prechecked_time {
                final_vec.push((
                    convert_time_to_unit_string(prechecked_time),
                    get_color(prechecked_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(applied_time) = self.applied_time {
                final_vec.push((
                    convert_time_to_unit_string(applied_time),
                    get_color(applied_time),
                ));
            } else {
                final_vec.push(missing_value.clone());
            }

            if let Some(broadcast_time) = self.broadcast_time {
                final_vec.push((
                    convert_time_to_unit_string(broadcast_time),
                    get_color(broadcast_time),
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
            EndorsementState::Missing => String::from("missing"),
            EndorsementState::Broadcast => String::from("broadcast"),
            EndorsementState::Applied => String::from("broadcast"),
            EndorsementState::Prechecked => String::from("prechecked"),
            EndorsementState::Decoded => String::from("decoded"),
            EndorsementState::Received => String::from("received"),
        }
    }
}

impl Default for EndorsementState {
    fn default() -> Self {
        Self::Missing
    }
}