
use tokio_tungstenite::connect_async;
use tokio::task::JoinHandle;
use futures_util::stream::StreamExt;

use crate::model::StateRef;

pub async fn spawn_ws_reader(state: StateRef) -> JoinHandle<()> {
    tokio::spawn(async move {
        let (ws_stream, _) = connect_async("ws://116.202.128.230:4927").await.expect("Failed to connect");
        let (_, read) = ws_stream.split();
        let state = state.clone();

        read.for_each(|raw_message| async {
            raw_message.and_then(|message| {
                Ok(serde_json::from_str(&message.to_string()).and_then(|data: serde_json::Value| {
                    Ok(data.as_array().and_then(|message_array| {
                        Some(for message in message_array {
                            message.clone().as_object().and_then(|message_obj| {
                                message_obj["type"].as_str().and_then(|type_str| {
                                    let mut state = state.write().unwrap();
                                    match type_str {
                                        "incomingTransfer" => {
                                            Some(state.update_incoming_transfer(serde_json::from_value(message["payload"].clone()).unwrap()))
                                        },
                                        "peersMetrics" => {
                                            Some(state.update_peer_metrics(serde_json::from_value(message["payload"].clone()).unwrap()))
                                        }
                                        "blockApplicationStatus" => {
                                            Some(state.update_application_status(serde_json::from_value(message["payload"].clone()).unwrap()))
                                        }
                                        _ => None,
                                    }
                                })
                            });
                        })
                    }))
                }))
            }).ok();
        }).await
    })
}