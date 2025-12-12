use std::{collections::HashMap, sync::Arc};
use axum::extract::ws::Message;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
// disclaimer: this was written with the help of copilot
/// A handle to send messages to a connected client.
#[derive(Clone)]
pub struct ClientTx {
    pub tx: mpsc::UnboundedSender<Message>,
}

/// Registry of connected clients by UUID.
#[derive(Default, Clone)]
pub struct WsHub {
    clients: Arc<RwLock<HashMap<Uuid, ClientTx>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a client with its sending channel.
    pub async fn register(&self, id: Uuid, tx: ClientTx) {
        self.clients.write().await.insert(id, tx);
    }

    /// Unregister a client, removing it from the hub.
    pub async fn unregister(&self, id: &Uuid) {
        self.clients.write().await.remove(id);
    }

    /// Send a message to a specific client. Returns true if the client exists.
    pub async fn send_to(&self, id: &Uuid, msg: Message) -> bool {
        if let Some(client) = self.clients.read().await.get(id) {
            let _ = client.tx.send(msg);
            true
        } else {
            false
        }
    }

    /// Broadcast a message to all connected clients.
    pub async fn broadcast(&self, msg: Message) {
        let clients = self.clients.read().await;
        for (_, client) in clients.iter() {
            let _ = client.tx.send(msg.clone());
        }
    }

    /// Current number of connected clients (for metrics).
    pub async fn len(&self) -> usize {
        self.clients.read().await.len()
    }
}