use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

pub type OperationsStats = BTreeMap<String, OperationStats>;

#[derive(Deserialize, Clone, Debug)]
pub struct OperationStats {
    kind: Option<OperationKind>,
    /// Minimum time when we saw this operation. Latencies are measured
    /// from this point.
    min_time: Option<u64>,
    first_block_timestamp: Option<u64>,
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

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone, Copy)]
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
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum OperationValidationResult {
    Applied,
    Refused,
    BranchRefused,
    BranchDelayed,
}

pub struct StatisticsSortable {
    pub datetime: u64,
    pub hash: String,
    pub nodes: usize,
    pub delta: i128,
    pub received: i128,
    pub content_received: i128,
    pub validation_started: i128,
    pub preaplly_started: i128,
    pub preapply_ended: i128,
    pub validation_finished: i128,
    pub vallen: i128,
    pub sent: i128,
    pub kind: String,
}
