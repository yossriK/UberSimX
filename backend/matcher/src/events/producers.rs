//  publishes outgoing events (MatchProposed, MatchConfirmed, etc)
use serde::Serialize;
use std::sync::Arc;
use common::ubersimx_messaging::{messagingclient::MessagingClient, Messaging};

/// Event producer publishes domain events back to NATS
#[derive(Clone)]
pub struct EventProducer {
    nc: Arc<MessagingClient>,
}

impl EventProducer {
    pub fn new(nc: Arc<MessagingClient>) -> Self {
        Self { nc }
    }

    /// Generic publish helper
    #[allow(dead_code)]
    pub async fn publish<T: Serialize>(&self, subject: &str, evt: &T) -> anyhow::Result<()> {
        let payload = serde_json::to_vec(evt).expect("failed to serialize event");
        self.nc.publish(String::from(subject), payload).await
    }
}
