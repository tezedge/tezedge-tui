use std::{
    collections::BTreeMap,
    hash::Hash,
    str::FromStr,
};

use serde::Deserialize;

pub type EndorsementRights = BTreeMap<String, Vec<u32>>;
pub type EndorsementStatuses = BTreeMap<String, EndorsementStatus>;

// TODO: update accordingly
pub type EndorsementRightsTableData = Vec<Vec<String>>;

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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EndorsementState {
    Missing = 0,
    Broadcast = 1,
    Applied = 2,
    Prechecked = 3,
    Decoded = 4,
    Received = 5,
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

#[derive(Clone, Debug, Default)]
pub struct EndorsementStatusSortable {
    pub delta: u64,
    pub decoded_time: u64,
    pub received_time: u64,
    pub applied_time: u64,
    pub prechecked_time: u64,
    pub broadcast_time: u64,
    pub slot: u32,
    pub state: EndorsementState,

    pub baker: String,
    pub slot_count: usize,
}

impl EndorsementStatus {
    pub fn to_sortable_descending(
        &self,
        baker: String,
        slot_count: usize,
    ) -> EndorsementStatusSortable {
        let delta =
            if let (Some(broadcast), Some(received)) = (self.broadcast_time, self.received_time) {
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
            state: EndorsementState::from_str(&self.state).unwrap_or_default(),
        }
    }

    pub fn to_sortable_ascending(
        &self,
        baker: String,
        slot_count: usize,
    ) -> EndorsementStatusSortable {
        let delta =
            if let (Some(broadcast), Some(received)) = (self.broadcast_time, self.received_time) {
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
            state: EndorsementState::from_str(&self.state).unwrap_or_default(),
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

    pub fn construct_tui_table_data(&self) -> Vec<String> {
        let mut final_vec = Vec::with_capacity(9);

        final_vec.push(self.slot_count.to_string());
        final_vec.push(self.baker.clone());
        final_vec.push(self.state.to_string());

        if self.broadcast_time != 0
            && self.received_time != 0
            && self.broadcast_time != u64::MAX
            && self.received_time != u64::MAX
        {
            final_vec.push(convert_time_to_unit_string(
                self.broadcast_time - self.received_time,
            ))
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
