pub use redux_rs::TimeService;

pub mod rpc_service;

pub mod ws_service;

pub mod tui_service;

pub mod service_async_channel;
pub use service_async_channel::*;

use self::{
    rpc_service::{RpcService, RpcServiceDefault},
    tui_service::{TuiService, TuiServiceDefault},
    ws_service::{WebsocketService, WebsocketServiceDefault},
};

pub trait Service: TimeService {
    type Rpc: RpcService;
    type Tui: TuiService;
    type Ws: WebsocketService;

    fn rpc(&mut self) -> &mut Self::Rpc;
    fn tui(&mut self) -> &mut Self::Tui;
    fn ws(&mut self) -> &mut Self::Ws;
}

pub struct ServiceDefault {
    pub rpc: RpcServiceDefault,
    pub tui: TuiServiceDefault,
    pub ws: WebsocketServiceDefault,
}

impl TimeService for ServiceDefault {}

impl Service for ServiceDefault {
    type Rpc = RpcServiceDefault;
    type Tui = TuiServiceDefault;
    type Ws = WebsocketServiceDefault;

    fn rpc(&mut self) -> &mut Self::Rpc {
        &mut self.rpc
    }
    fn tui(&mut self) -> &mut Self::Tui {
        &mut self.tui
    }
    fn ws(&mut self) -> &mut Self::Ws {
        &mut self.ws
    }
}
