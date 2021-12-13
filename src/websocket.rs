use futures_util::stream::StreamExt;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use thiserror::Error;

use crate::model::StateRef;

#[derive(Debug, Error)]
pub enum WebsocketError {
    #[error("Failed to connect to: {url}")]
    ConnectionError { url: &'static str },
    #[error("Failed to deserialize websocket message")]
    DeserializeError,
}

pub async fn spawn_ws_reader(state: StateRef) -> Result<JoinHandle<()>, WebsocketError> {
    let handle = tokio::spawn(async move {
        let (ws_stream, _) = connect_async("ws://116.202.128.230:4927")
            .await
            .expect("Failed to connect");
        let (_, read) = ws_stream.split();
        let state = state.clone();

        read.for_each(|raw_message| async {
            raw_message
                .map(|message| {
                    serde_json::from_str(&message.to_string()).map(|data: serde_json::Value| {
                        data.as_array().map(|message_array| {
                            for message in message_array {
                                message.clone().as_object().map(|message_obj| {
                                    message_obj["type"].as_str().map(|type_str| {
                                        let mut state = state.write().unwrap();
                                        match type_str {
                                            "incomingTransfer" => {
                                                if let Ok(data) = serde_json::from_value(message["payload"].clone()) {
                                                    state.update_incoming_transfer(data);
                                                } else {
                                                    // Error
                                                }
                                            },
                                            "peersMetrics" => state.update_peer_metrics(
                                                serde_json::from_value(message["payload"].clone())
                                                    .unwrap(),
                                            ),
                                            "blockApplicationStatus" => state
                                                .update_application_status(
                                                    serde_json::from_value(
                                                        message["payload"].clone(),
                                                    )
                                                    .unwrap(),
                                                ),
                                            "blockStatus" => state.update_block_metrics(
                                                serde_json::from_value(message["payload"].clone())
                                                    .unwrap(),
                                            ),
                                            "chainStatus" => state.update_cycle_data(
                                                serde_json::from_value(message["payload"].clone())
                                                    .unwrap(),
                                            ),
                                            _ => {}
                                        }
                                    })
                                });
                            }
                        })
                    })
                })
                .ok();
        })
        .await;
    });

    Ok(handle)
}
