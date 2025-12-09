// "listeners": NATS subscriptions → stream incoming events
// goal is to keep subscriptions/producers isolated from the rest of the app

use std::sync::Arc;

use common::{events_schema::RideRequestedEvent, subjects::RIDE_REQUESTED_SUBJECT};
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use ubersimx_messaging::{messagingclient::MessagingClient, Messaging};

use crate::{events::handler::EventHandler, matcher::service::MatcherService};

pub struct Consumers {
    messaging_client: Arc<MessagingClient>,
}

impl Consumers {
    pub fn new(mc: Arc<MessagingClient>) -> Self {
        Self {
            messaging_client: mc,
        }
    }

    /// Low-level subscribe helper
    pub async fn subscribe<T, H>(&self, subject: &str, handler: Arc<H>)
    where
        T: DeserializeOwned + Send + 'static,
        H: EventHandler<T> + Send + Sync + 'static,
    {
        let sub = self
            .messaging_client
            .subscribe(String::from(subject))
            .await
            .unwrap();

        tokio::spawn(async move {
            let mut stream = sub;
            while let Some(msg) = stream.next().await {
                if let Ok(msg) = msg {
                    match serde_json::from_slice::<T>(&msg.data) {
                        Ok(evt) => handler.handle(evt).await,
                        Err(_) => todo!(), //tracing::error!("Failed to parse event on {}: {:?}", subject, e),
                    }
                }
            }
        });
    }

    /// High-level helper: registers all event consumers for MatcherService
    pub async fn register_all(&self, matcher: Arc<MatcherService>) {
        self.subscribe::<RideRequestedEvent, _>(RIDE_REQUESTED_SUBJECT, matcher.clone())
            .await;
    }
}

// This was the old code before refactoring to generic subscribe method
// pub async fn subscribe_listeners(nc: Arc<MessagingClient>, handler: Arc<EventHandler>) {
//     let mut sub = nc.subscribe(String::from("rider.requested")).await.unwrap();

//     tokio::spawn(async move {
//         while let Some(msg) = sub.next().await {
//             if let Ok(msg) = msg {
//                 if let Ok(event) = serde_json::from_slice::<RideRequested>(&msg.data) {
//                     handler.on_ride_requested(event).await;
//                 }
//             }
//         }
//     });

//     // add more consumers
// }
