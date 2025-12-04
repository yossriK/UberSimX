use crate::{Message, Messaging};
use async_nats;
use async_trait::async_trait;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;

pub struct MessagingClient {
    client: Arc<async_nats::Client>,
}

impl MessagingClient {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let client = async_nats::connect(url).await?;
        Ok(MessagingClient {
            client: Arc::new(client),
        })
    }
}

#[async_trait]
impl Messaging for MessagingClient {
    async fn publish(&self, subject: String, data: Vec<u8>) -> anyhow::Result<()> {
        self.client.publish(subject, data.into()).await?;
        Ok(())
    }
    async fn subscribe(
        &self,
        subject: String,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = anyhow::Result<Message>> + Send>>> {
        let sub = self.client.subscribe(subject).await?;
        let stream = sub.map(|msg| {
            Ok(Message {
                subject: msg.subject.to_string(),
                data: msg.payload.to_vec(),
            })
        });
        Ok(Box::pin(stream))
    }
    async fn request(&self, subject: String, data: Vec<u8>) -> anyhow::Result<Message> {
        let msg = self.client.request(subject, data.into()).await?;
        Ok(Message {
            subject: msg.subject.to_string(),
            data: msg.payload.to_vec(),
        })
    }
}
