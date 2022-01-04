use std::fmt::Display;

use reqwest::Response;
use slog::{Logger, info};
use thiserror::Error;
use url::Url;

use crate::model::{CurrentHeadHeader, EndorsementRights, EndorsementStatuses, OperationsStats};

#[derive(Clone, Debug)]
pub struct Node {
    pub url: Url,

    pub log: Logger,
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

#[allow(clippy::large_enum_variant)]
pub enum RpcResponse {
    EndorsementRights(EndorsementRights),
    EndorsementsStatus(EndorsementStatuses),
    CurrentHeadHeader(CurrentHeadHeader),
    OperationsStats(OperationsStats),
}

impl Node {
    pub fn new(url: &Url, log: Logger) -> Self {
        Self {
            url: url.clone(),
            log,
        }
    }
    pub async fn call_rpc(
        &self,
        rpc: RpcCall,
        query_arg: Option<&str>,
    ) -> Result<RpcResponse, RpcError> {
        let res = self.call_rpc_inner(rpc, query_arg).await?;

        match rpc {
            RpcCall::EndorsementRights => {
                let rights: EndorsementRights = res.json().await.map_err(|e| RpcError::RequestErrorDetailed(rpc, e))?;
                Ok(RpcResponse::EndorsementRights(rights))
            }
            RpcCall::CurrentHeadHeader => {
                let header: CurrentHeadHeader = res.json().await.map_err(|e| RpcError::RequestErrorDetailed(rpc, e))?;
                Ok(RpcResponse::CurrentHeadHeader(header))
            }
            RpcCall::EndersementsStatus => {
                let statuses: EndorsementStatuses = res.json().await.map_err(|e| RpcError::RequestErrorDetailed(rpc, e))?;
                Ok(RpcResponse::EndorsementsStatus(statuses))
            }
            RpcCall::OperationsStats => {
                let stats: OperationsStats = res.json().await.map_err(|e| RpcError::RequestErrorDetailed(rpc, e))?;
                Ok(RpcResponse::OperationsStats(stats))
            }
        }
    }

    async fn call_rpc_inner(
        &self,
        rpc: RpcCall,
        query_arg: Option<&str>,
    ) -> Result<Response, RpcError> {
        let mut url = self.url.join(rpc.to_url())?;
        if let Some(query) = query_arg {
            url = url.join(query)?;
        }

        reqwest::get(url).await.map_err(|e| RpcError::RequestErrorDetailed(rpc, e))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RpcCall {
    EndorsementRights,
    EndersementsStatus,
    CurrentHeadHeader,
    OperationsStats,
}

impl Display for RpcCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcCall::EndorsementRights => write!(f, "EndorsementRights"),
            RpcCall::EndersementsStatus => write!(f, "EndersementsStatus"),
            RpcCall::CurrentHeadHeader => write!(f, "CurrentHeadHeader"),
            RpcCall::OperationsStats => write!(f, "OperationsStats"),
        }
    }
}

impl RpcCall {
    pub fn to_url(&self) -> &str {
        match self {
            RpcCall::EndorsementRights => "dev/shell/automaton/endorsing_rights",
            RpcCall::EndersementsStatus => "dev/shell/automaton/endorsements_status",
            RpcCall::CurrentHeadHeader => "chains/main/blocks/head/header",
            RpcCall::OperationsStats => "dev/shell/automaton/mempool/operation_stats",
        }
    }
}
