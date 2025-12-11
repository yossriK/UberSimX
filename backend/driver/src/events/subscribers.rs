// "subscribers": NATS subscriptions → stream incoming events
// goal is to keep subscriptions/producers isolated from the rest of the app
// these are not concrete implementation as we are usng messagingCLinet that wraps around NATS.
// else we would have all the events logic placed under infrastructure. This module is application
// orchestration not infra plumbing.

use std::sync::Arc;

use common::{
    events_schema::DriverAssignedRideEvent,
    subjects::{DRIVER_ASSIGNED_SUBJECT},
};
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use ubersimx_messaging::{messagingclient::MessagingClient, Messaging};

use crate::{events::handlers::EventHandler, service::ride_lifecycle::RideLifeCycleService};

pub struct Subscribers {
    messaging_client: Arc<MessagingClient>,
}

impl Subscribers {
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
                        Err(e) => {
                            eprintln!("Failed to parse event on {:?}: {:?}", &msg.data, e);
                            todo!()
                        }
                    }
                }
            }
        });
    }

    /// High-level helper: registers all event consumers for Ride Events
    pub async fn register_ride_evnets_consumers(&self, matcher: Arc<RideLifeCycleService>) {
        self.subscribe::<DriverAssignedRideEvent, _>(DRIVER_ASSIGNED_SUBJECT, matcher.clone())
            .await;
    }
}
