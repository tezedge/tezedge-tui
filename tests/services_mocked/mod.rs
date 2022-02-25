pub use redux_rs::TimeService;
use tezedge_tui::services::Service;
use tui::backend::TestBackend;

use self::{
    rpc_service_async::RpcServiceMocked, tui_service::TuiServiceMocked,
    ws_service::WebsocketServiceMocked,
};

pub mod rpc_service_async;

pub mod ws_service;

pub mod tui_service;

pub struct ServiceMocked {
    pub rpc: RpcServiceMocked,
    pub tui: TuiServiceMocked,
    pub ws: WebsocketServiceMocked,
}

impl TimeService for ServiceMocked {}

impl Service for ServiceMocked {
    type Be = TestBackend;
    type Rpc = RpcServiceMocked;
    type Tui = TuiServiceMocked;
    type Ws = WebsocketServiceMocked;

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
