//  publishes outgoing events (MatchProposed, MatchConfirmed, etc)
// technically this won't needed but I don't want hte services/business logic to depend on the messaging client directly
use std::sync::Arc;
use ubersimx_messaging::{messagingclient::MessagingClient, Messaging};

/// Event producer publishes domain events back to NATS
#[derive(Clone)]
pub struct EventPublisher {
    nc: Arc<MessagingClient>,
}

impl EventPublisher {
    pub fn new(nc: Arc<MessagingClient>) -> Self {
        Self { nc }
    }

    /// Generic publish helper
    pub async fn publish(&self, subject: &str, payload: Vec<u8>) -> anyhow::Result<()> {
        self.nc.publish(String::from(subject), payload).await
    }
}
