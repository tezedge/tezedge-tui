use std::fmt::Display;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use slog::{info, warn, Logger};
use thiserror::Error;
use time::OffsetDateTime;
use tokio::sync::mpsc;
use url::Url;

use crate::{
    baking::{BakingRightsPerLevel, BlockApplicationStatistics, PerPeerBlockStatisticsVector},
    endorsements::{
        EndorsementRights, EndorsementRightsWithTimePerLevel, EndorsementStatuses,
        MempoolEndorsementStats,
    },
    operations::OperationsStats,
};

use super::RequestTrySendError;

pub type RpcRecvError = mpsc::error::TryRecvError;

#[async_trait]
pub trait RpcService {
    fn request_send(&mut self, req: RpcCall) -> Result<(), RequestTrySendError<RpcCall>>;
    async fn response_recv(&mut self) -> Option<RpcResponse>;
}

#[derive(Debug)]
pub struct RpcServiceDefault {
    sender: mpsc::Sender<RpcCall>,
    receiver: mpsc::Receiver<RpcResponse>,
    _url: Url,
}

impl RpcServiceDefault {
    pub fn new(bound: usize, url: Url, log: &Logger) -> Self {
        let (call_tx, call_rx) = mpsc::channel(bound);
        let (response_tx, response_rx) = mpsc::channel(bound);

        let t_url = url.clone();
        let t_log = log.clone();

        tokio::task::spawn(
            async move { Self::run_worker(call_rx, response_tx, &t_url, &t_log).await },
        );

        Self {
            sender: call_tx,
            receiver: response_rx,
            _url: url,
        }
    }
}

#[async_trait]
impl RpcService for RpcServiceDefault {
    fn request_send(&mut self, req: RpcCall) -> Result<(), RequestTrySendError<RpcCall>> {
        self.sender.try_send(req)
    }

    async fn response_recv(&mut self) -> Option<RpcResponse> {
        self.receiver.recv().await
    }
}

impl RpcServiceDefault {
    async fn run_worker(
        mut call_receiver: mpsc::Receiver<RpcCall>,
        response_sender: mpsc::Sender<RpcResponse>,
        url: &Url,
        log: &Logger,
    ) {
        info!(log, "Rpc service started. Rpc url: {}", url);
        while let Some(req) = call_receiver.recv().await {
            match Self::call_rpc(req, url).await {
                Ok(response) => {
                    let _ = response_sender.send(response).await;
                }
                Err(e) => {
                    warn!(log, "Rpc failed: {}", e)
                }
            };
        }
    }

    async fn call_rpc(request: RpcCall, url: &Url) -> Result<RpcResponse, RpcError> {
        let mut url = url.join(request.to_url()).unwrap();
        if let Some(query) = request.query_arg.clone() {
            url = url.join(&query).unwrap();
        }

        let response = reqwest::get(url)
            .await
            .map_err(|e| RpcError::RequestErrorDetailed(request.clone(), e))?;

        match request.target {
            RpcTarget::EndorsementRights => {
                let rights: EndorsementRights = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::EndorsementRights(rights))
            }
            RpcTarget::CurrentHeadHeader => {
                let header: CurrentHeadHeader = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::CurrentHeadHeader(header))
            }
            RpcTarget::EndersementsStatus => {
                let statuses: EndorsementStatuses = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::EndorsementsStatus(statuses))
            }
            RpcTarget::OperationsStats => {
                let stats: OperationsStats = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::OperationsStats(stats))
            }
            RpcTarget::ApplicationStatistics => {
                let stats: Vec<BlockApplicationStatistics> = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::ApplicationStatistics(stats))
            }
            RpcTarget::PerPeerBlockStatistics => {
                let stats: PerPeerBlockStatisticsVector = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::PerPeerBlockStatistics(stats))
            }
            RpcTarget::BakingRights => {
                let rights: Vec<BakingRightsPerLevel> = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::BakingRights(rights))
            }
            RpcTarget::EndorsementRightsWithTime => {
                let rights: Vec<EndorsementRightsWithTimePerLevel> = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::EndorsementRightsWithTime(rights))
            }
            RpcTarget::MempoolEndorsementStats => {
                let stats: MempoolEndorsementStats = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::MempoolEndorsementStats(stats))
            }
            RpcTarget::NetworkConstants => {
                let constants: NetworkConstants = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::NetworkConstants(constants))
            }
            RpcTarget::CurrentHeadMetadata => {
                let metadata: CurrentHeadMetadata = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::CurrentHeadMetadata(metadata))
            }
            RpcTarget::BestRemoteLevel => {
                let level: Option<i32> = response
                    .json()
                    .await
                    .map_err(|e| RpcError::RequestErrorDetailed(request, e))?;
                Ok(RpcResponse::BestRemoteLevel(level))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("Error while parsing URL: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Error while calling RPC {0}: {1}")]
    RequestErrorDetailed(RpcCall, reqwest::Error),
    #[error("Error while desierializing RPC response: {0}")]
    DeserializationError(#[from] serde_json::Error),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcCall {
    pub target: RpcTarget,
    query_arg: Option<String>,
}

impl RpcCall {
    pub fn new(target: RpcTarget, query_arg: Option<String>) -> Self {
        Self { target, query_arg }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RpcTarget {
    EndorsementRights,
    EndersementsStatus,
    CurrentHeadHeader,
    OperationsStats,
    ApplicationStatistics,
    PerPeerBlockStatistics,
    BakingRights,
    EndorsementRightsWithTime,
    MempoolEndorsementStats,
    NetworkConstants,
    CurrentHeadMetadata,
    BestRemoteLevel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum RpcResponse {
    EndorsementRights(EndorsementRights),
    EndorsementsStatus(EndorsementStatuses),
    CurrentHeadHeader(CurrentHeadHeader),
    OperationsStats(OperationsStats),
    ApplicationStatistics(Vec<BlockApplicationStatistics>),
    PerPeerBlockStatistics(PerPeerBlockStatisticsVector),
    BakingRights(Vec<BakingRightsPerLevel>),
    EndorsementRightsWithTime(Vec<EndorsementRightsWithTimePerLevel>),
    MempoolEndorsementStats(MempoolEndorsementStats),
    NetworkConstants(NetworkConstants),
    CurrentHeadMetadata(CurrentHeadMetadata),
    BestRemoteLevel(Option<i32>),
}

impl Display for RpcCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.target {
            RpcTarget::EndorsementRights => {
                write!(f, "EndorsementRights - Query args: {:?}", self.query_arg)
            }
            RpcTarget::EndersementsStatus => {
                write!(f, "EndersementsStatus - Query args: {:?}", self.query_arg)
            }
            RpcTarget::CurrentHeadHeader => {
                write!(f, "CurrentHeadHeader - Query args: {:?}", self.query_arg)
            }
            RpcTarget::OperationsStats => {
                write!(f, "OperationsStats - Query args: {:?}", self.query_arg)
            }
            RpcTarget::ApplicationStatistics => {
                write!(
                    f,
                    "ApplicationStatistics - Query args: {:?}",
                    self.query_arg
                )
            }
            RpcTarget::PerPeerBlockStatistics => {
                write!(
                    f,
                    "PerPeerBlockStatistics - Query args: {:?}",
                    self.query_arg
                )
            }
            RpcTarget::BakingRights => {
                write!(f, "BakingRights - Query args: {:?}", self.query_arg)
            }
            RpcTarget::EndorsementRightsWithTime => {
                write!(
                    f,
                    "EndorsementRightsWithTime - Query args: {:?}",
                    self.query_arg
                )
            }
            RpcTarget::MempoolEndorsementStats => {
                write!(
                    f,
                    "MempoolEndorsementStats - Query args: {:?}",
                    self.query_arg
                )
            }
            RpcTarget::NetworkConstants => {
                write!(f, "NetworkConstants - Query args: {:?}", self.query_arg)
            }
            RpcTarget::CurrentHeadMetadata => {
                write!(f, "CurrentHeadMetadata - Query args: {:?}", self.query_arg)
            }
            RpcTarget::BestRemoteLevel => {
                write!(f, "BestRemoteLevel - Query args: {:?}", self.query_arg)
            }
        }
    }
}

impl RpcCall {
    pub fn to_url(&self) -> &str {
        match self.target {
            RpcTarget::EndorsementRights => "dev/shell/automaton/endorsing_rights",
            RpcTarget::EndersementsStatus => "dev/shell/automaton/endorsements_status",
            RpcTarget::CurrentHeadHeader => "chains/main/blocks/head/header",
            RpcTarget::OperationsStats => "dev/shell/automaton/mempool/operation_stats",
            RpcTarget::ApplicationStatistics => {
                "dev/shell/automaton/stats/current_head/application"
            }
            RpcTarget::PerPeerBlockStatistics => "dev/shell/automaton/stats/current_head/peers",
            RpcTarget::BakingRights => "chains/main/blocks/head/helpers/baking_rights",
            RpcTarget::EndorsementRightsWithTime => {
                "chains/main/blocks/head/helpers/endorsing_rights"
            }
            RpcTarget::MempoolEndorsementStats => "dev/shell/automaton/stats/mempool/endorsements",
            RpcTarget::NetworkConstants => "chains/main/blocks/head/context/constants",
            RpcTarget::CurrentHeadMetadata => "chains/main/blocks/head/metadata",
            RpcTarget::BestRemoteLevel => "dev/peers/best_remote_level",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CurrentHeadHeader {
    pub level: i32,
    pub hash: String,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
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

// TODO: the unwrap in OffsetDateTime
impl Default for CurrentHeadHeader {
    fn default() -> Self {
        Self {
            level: Default::default(),
            hash: Default::default(),
            timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            chain_id: Default::default(),
            predecessor: Default::default(),
            validation_pass: Default::default(),
            operations_hash: Default::default(),
            fitness: Default::default(),
            context: Default::default(),
            protocol: Default::default(),
            signature: Default::default(),
            priority: Default::default(),
            proof_of_work_nonce: Default::default(),
            liquidity_baking_escape_vote: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct NetworkConstants {
    // we only need this one for now
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub minimal_block_delay: i32,
    pub preserved_cycles: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CurrentHeadMetadata {
    pub level_info: LevelInfo,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct LevelInfo {
    pub cycle: i32,
    cycle_position: i32,
    expected_commitment: bool,
    pub level: i32,
    level_position: i32,
}
