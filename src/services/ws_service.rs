use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use slog::{error, info, warn, Logger};
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::synchronization::{
    BlockApplicationStatus, BlockStatus, ChainStatus, IncomingTransferMetrics, PeerMetrics,
};

pub type WsRecvError = mpsc::error::TryRecvError;

pub type WsReceiver = mpsc::Receiver<Vec<WebsocketMessage>>;
pub type WsSender = mpsc::Sender<Vec<WebsocketMessage>>;

pub trait WebsocketService {
    fn message_try_recv(&mut self) -> Result<Vec<WebsocketMessage>, WsRecvError>;
}

impl WebsocketService for WebsocketServiceDefault {
    fn message_try_recv(&mut self) -> Result<Vec<WebsocketMessage>, WsRecvError> {
        self.receiver.try_recv()
    }
}

#[derive(Debug)]
pub struct WebsocketServiceDefault {
    receiver: WsReceiver,
}

impl WebsocketServiceDefault {
    pub fn new(bound: usize, websocket_url: Url, log: &Logger) -> Self {
        let (tx, rx) = mpsc::channel(bound);

        let t_log = log.clone();
        tokio::task::spawn(async move { Self::run_worker(websocket_url, tx, t_log).await });

        Self { receiver: rx }
    }
    async fn run_worker(websocket_url: Url, sender: WsSender, log: Logger) {
        let (ws_stream, _) = match connect_async(websocket_url.clone()).await {
            Ok((ws_stream, ws_response)) => (ws_stream, ws_response),
            Err(_) => {
                error!(log, "Failed to connect websocket at {}", websocket_url);
                return;
            }
        };

        info!(log, "Websocket connected: {}", websocket_url);

        let (_, ws_reader) = ws_stream.split();

        ws_reader
            .for_each(|raw_message| async {
                let sender = sender.clone();
                let log = log.clone();
                match raw_message {
                    Ok(message) => {
                        match serde_json::from_str::<Vec<WebsocketMessage>>(&message.to_string()) {
                            Ok(deserialized) => {
                                if let Err(e) = sender.send(deserialized).await {
                                    warn!(
                                        log,
                                        "Failed to send websocket message to state machine: {}", e
                                    )
                                }
                            }
                            Err(e) => {
                                warn!(log, "Failed to deserialze websocket message: {}", e);
                            }
                        };
                    }
                    Err(e) => {
                        warn!(log, "Failed to read from websocket: {}", e);
                    }
                }
            })
            .await;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "payload")]
pub enum WebsocketMessage {
    IncomingTransfer(IncomingTransferMetrics),
    BlockStatus(Vec<BlockStatus>),
    BlockApplicationStatus(BlockApplicationStatus),
    ChainStatus(ChainStatus),
    PeersMetrics(Vec<PeerMetrics>),
}

#[derive(Debug, Error)]
pub enum WebsocketError {
    #[error("Failed to connect to: {url}")]
    ConnectionError { url: &'static str },
    #[error("Failed to deserialize websocket message")]
    DeserializeError,
}
