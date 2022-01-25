use std::fmt::Display;

use serde::Deserialize;
use slog::{warn, Logger};
use thiserror::Error;
use tokio::sync::mpsc;
use url::Url;

use crate::{
    endorsements::{EndorsementRights, EndorsementStatuses},
    operations::OperationsStats,
};

use super::{
    worker_channel, RequestTrySendError, ResponseTryRecvError, ServiceWorkerAsyncRequester,
    ServiceWorkerAsyncResponder,
};

pub type RpcRecvError = mpsc::error::TryRecvError;

type RpcWorkerRequester = ServiceWorkerAsyncRequester<RpcCall, RpcResponse>;
type RpcWorkerResponder = ServiceWorkerAsyncResponder<RpcCall, RpcResponse>;

pub trait RpcService {
    fn request_send(&mut self, req: RpcCall) -> Result<(), RequestTrySendError<RpcCall>>;
    fn response_try_recv(&mut self) -> Result<RpcResponse, ResponseTryRecvError>;
}

#[derive(Debug)]
pub struct RpcServiceDefault {
    worker_channel: RpcWorkerRequester,
    _url: Url,
}

impl RpcServiceDefault {
    pub fn new(bound: usize, url: Url, log: &Logger) -> Self {
        let (requester, responder) = worker_channel(bound);

        let t_url = url.clone();
        let t_log = log.clone();
        // thread::Builder::new()
        //     .name("rpc-thread".to_owned())
        //     .spawn(move || Self::run_worker(responder, t_url))
        //     .unwrap();

        tokio::task::spawn(async move { Self::run_worker(responder, &t_url, &t_log).await });

        Self {
            worker_channel: requester,
            _url: url,
        }
    }
}

impl RpcService for RpcServiceDefault {
    fn request_send(&mut self, req: RpcCall) -> Result<(), RequestTrySendError<RpcCall>> {
        self.worker_channel.try_send(req)
    }

    fn response_try_recv(&mut self) -> Result<RpcResponse, ResponseTryRecvError> {
        self.worker_channel.try_recv()
    }
}

impl RpcServiceDefault {
    async fn run_worker(mut channel: RpcWorkerResponder, url: &Url, log: &Logger) {
        while let Ok(req) = channel.recv().await {
            match Self::call_rpc(req, url).await {
                Ok(response) => {
                    let _ = channel.send(response).await;
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

#[derive(Clone, Debug)]
pub struct RpcCall {
    pub target: RpcTarget,
    query_arg: Option<String>,
}

impl RpcCall {
    pub fn new(target: RpcTarget, query_arg: Option<String>) -> Self {
        Self { target, query_arg }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RpcTarget {
    EndorsementRights,
    EndersementsStatus,
    CurrentHeadHeader,
    OperationsStats,
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum RpcResponse {
    EndorsementRights(EndorsementRights),
    EndorsementsStatus(EndorsementStatuses),
    CurrentHeadHeader(CurrentHeadHeader),
    OperationsStats(OperationsStats),
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
        }
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
