use tezedge_tui::services::ws_service::{WebsocketMessage, WebsocketService, WsRecvError};

impl WebsocketService for WebsocketServiceMocked {
    fn message_try_recv(&mut self) -> Result<Vec<WebsocketMessage>, WsRecvError> {
        Ok(vec![])
    }
}

#[derive(Debug)]
pub struct WebsocketServiceMocked {}

impl WebsocketServiceMocked {
    pub fn _new() -> Self {
        Self {}
    }
}
