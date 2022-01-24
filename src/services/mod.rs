pub use redux_rs::TimeService;

pub mod rpc_service;

pub mod ws_service;

pub mod tui_service;

pub mod service_async_channel;
pub use service_async_channel::*;

use self::rpc_service::{RpcService, RpcServiceDefault};

pub trait Service: TimeService {
    type Rpc: RpcService;
    // type Ws: WsService;

    fn rpc(&mut self) -> &mut Self::Rpc;
    // fn ws(&mut self) -> &mut Self::Ws;
}

pub struct ServiceDefault {
    pub rpc: RpcServiceDefault,
}

impl TimeService for ServiceDefault {}

impl Service for ServiceDefault {
    type Rpc = RpcServiceDefault;

    fn rpc(&mut self) -> &mut Self::Rpc {
        &mut self.rpc
    }
}
