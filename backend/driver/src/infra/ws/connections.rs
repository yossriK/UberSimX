use axum::extract::ws::{Message, WebSocket};
use common::ws_schema::{DriverLocationV1, Envelope, WSMsgType};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use super::hub::{ClientTx, WsHub};
use crate::service::location_update::{LocationUpdate, LocationUpdateService};
// Disclaimer: this was written with the help of copilot

/// Handles a single WebSocket connection lifecycle:
/// - registers client in the hub
/// - forwards server messages to the socket
/// - reads client messages, processes them
/// - unregisters on disconnect
pub async fn ws_on_upgrade(
    socket: WebSocket,
    hub: Arc<WsHub>,
    location_update_service: Arc<LocationUpdateService>,
    client_id: Uuid,
) {
    // Channel from server components -> this connection
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    hub.register(client_id, ClientTx { tx }).await;

    // Optional: greet the client
    let _ = hub
        .send_to(
            &client_id,
            Message::Text(
                serde_json::json!({
                    "type": "ws.welcome",
                    "client_id": client_id.to_string(),
                    "ts": chrono::Utc::now().timestamp_millis(),
                })
                .to_string()
                .into(),
            ),
        )
        .await;

    // Split the socket into sender/receiver halves
    let (mut ws_tx, mut ws_rx) = socket.split();

    // Task: forward server-pushed messages to the client
    let forward_task = {
        let client_id = client_id;
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // If the client side is closed, stop forwarding
                if ws_tx.send(msg).await.is_err() {
                    break;
                }
            }
            // Optionally: log that forwarding ended for client_id
            eprintln!("Forwarding task ended for client_id: {}", client_id);
        })
    };

    // Task: read messages from the client and handle them
    let read_task = {
        let hub = hub.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = ws_rx.next().await {
                match msg {
                    Message::Text(text) => {
                        // Example: route by "type" field in JSON
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                            let _message: String =
                                WSMsgType::DriverLocationUpdate.to_string();

                            match v.get("type").and_then(|t| t.as_str()) {
                                Some("driver_location_update") => {
                                    if let Ok(env) = serde_json::from_value::<Envelope<DriverLocationV1>>(v.clone()) {
                                        location_update_service
                                            .handle_location_update(
                                                client_id,
                                                env.data.latitude,
                                                env.data.longitude,
                                            )
                                            .await;
                                    }
                                }
                                Some("client.ping") => {
                                    let _ = hub
                                        .send_to(
                                            &client_id,
                                            Message::Text("{\"type\":\"server.pong\"}".into()),
                                        )
                                        .await;
                                }
                                _ => {
                                    // Unknown type; you may log or ignore
                                }
                            }
                        }
                    }
                    Message::Binary(_bytes) => {
                        // Handle binary frames if you use protobuf/CBOR
                    }
                    Message::Ping(data) => {
                        // Respond with Pong
                        let _ = hub.send_to(&client_id, Message::Pong(data)).await;
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }
        })
    };

    // Wait until either task finishes; then clean up
    let _ = tokio::join!(forward_task, read_task);
    hub.unregister(&client_id).await;
}
