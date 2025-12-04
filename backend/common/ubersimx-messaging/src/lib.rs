use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
pub mod messagingclient;
pub mod subjects;

/// Application-level message type
pub struct Message {
    pub subject: String,
    pub data: Vec<u8>,
}

/// Trait for messaging abstraction
#[async_trait]
pub trait Messaging: Send + Sync {
    async fn publish(&self, subject: String, data: Vec<u8>) -> anyhow::Result<()>;
    async fn subscribe(
        &self,
        subject: String,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = anyhow::Result<Message>> + Send>>>;
    async fn request(&self, subject: String, data: Vec<u8>) -> anyhow::Result<Message>;
}
