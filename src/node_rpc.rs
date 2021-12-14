use std::{collections::HashMap, str::FromStr};

use reqwest::Response;
use thiserror::Error;
use url::Url;

use crate::model::{CurrentHeadHeader, EndorsementRights};

#[derive(Clone, Debug)]
pub struct Node {
    pub url: Url,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            url: Url::from_str("http://127.0.0.1:18732").unwrap(),
        }
    }
}

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("Error while parsing URL: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Error while calling RPC: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Error while desierializing RPC response: {0}")]
    DeserializationError(#[from] serde_json::Error),
}

#[allow(clippy::large_enum_variant)]
pub enum RpcResponse {
    EndorsementRights(EndorsementRights),
    EndorsementsStatus,
    CurrentHeadHeader(CurrentHeadHeader),
}

impl Node {
    pub fn new(path: &str) -> Result<Self, url::ParseError> {
        let url = Url::from_str(path)?;

        Ok(Self { url })
    }
    pub async fn call_rpc(
        &self,
        rpc: RpcCall,
        query_arg: Option<&str>,
    ) -> Result<RpcResponse, RpcError> {
        let res = self.call_rpc_inner(rpc, query_arg).await?;

        match rpc {
            RpcCall::EndorsementRights => {
                let rights: EndorsementRights = res.json().await?;
                Ok(RpcResponse::EndorsementRights(rights))
            }
            RpcCall::CurrentHeadHeader => {
                let header: CurrentHeadHeader = res.json().await?;
                Ok(RpcResponse::CurrentHeadHeader(header))
            }
            _ => unimplemented!(),
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
        Ok(reqwest::get(url).await?)
    }
}

#[derive(Copy, Clone)]
pub enum RpcCall {
    EndorsementRights,
    EndersementsStatus,
    CurrentHeadHeader,
}

impl RpcCall {
    pub fn to_url(&self) -> &str {
        match self {
            RpcCall::EndorsementRights => "dev/shell/automaton/endorsing_rights",
            RpcCall::EndersementsStatus => "dev/shell/automaton/endorsements_status",
            RpcCall::CurrentHeadHeader => "chains/main/blocks/head/header",
        }
    }
}
