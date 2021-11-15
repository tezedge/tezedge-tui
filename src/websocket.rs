use tokio_tungstenite::connect_async;
use tokio::task::JoinHandle;
use futures_util::stream::StreamExt;

use crate::model::StateRef;

// TODO: refactor this
pub async fn spawn_ws_reader(state: StateRef) -> JoinHandle<()>{
    tokio::spawn(async move {
        let (ws_stream, _) = connect_async("ws://116.202.128.230:4927").await.expect("Failed to connect");
        let (_, read) = ws_stream.split();
        let state = state.clone();
        read.for_each(|raw_message| async {
            let data: serde_json::Value = serde_json::from_str(&raw_message.unwrap().to_string()).unwrap();
            if let Some(messages_array) = data.as_array() {
                for message in messages_array {
                    if let Some(message_obj) = message.clone().as_object() {
                        if let Some(str) = &message_obj["type"].as_str() {
                            let mut state = state.write().unwrap();
                            match *str {
                                "incomingTransfer" => {
                                    state.update_incoming_transfer(serde_json::from_value(message["payload"].clone()).unwrap())
                                },
                                "peersMetrics" => {
                                    // TODO
                                }
                                "blockApplicationStatus" => {
                                    state.update_application_status(serde_json::from_value(message["payload"].clone()).unwrap())
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }).await;
    })
} 