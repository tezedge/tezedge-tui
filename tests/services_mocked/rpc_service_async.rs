use async_trait::async_trait;
use tezedge_tui::services::{rpc_service_async::{RpcService, RpcCall, RpcResponse}, RequestTrySendError};

#[derive(Debug)]
pub struct RpcServiceMocked {}

impl RpcServiceMocked {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl RpcService for RpcServiceMocked {
    fn request_send(&mut self, req: RpcCall) -> Result<(), RequestTrySendError<RpcCall>> {
        Ok(())
    }

    async fn response_recv(&mut self) -> Option<RpcResponse> {
        None
    }
}