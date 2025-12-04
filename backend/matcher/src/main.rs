mod api;
mod events;
mod matcher;

use std::{env, sync::Arc};

use anyhow::Ok;
use common::ubersimx_messaging::messagingclient::MessagingClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::from_filename("settings.env").ok();

    let messaging_url = env::var("MESSAGING_URL")
        .map_err(|e| anyhow::anyhow!("MESSAGING_URL must be set in .env: {}", e))?;

    let redis_url = env::var("REDIS_URL")
        .map_err(|e| anyhow::anyhow!("REDIS_URL must be set in .env: {}", e))?;

    // Connect to the messaging service
    // todo properly configure the URL via env var or config file and handle the error
    let messaging_client = Arc::new(MessagingClient::connect(&messaging_url).await.unwrap());

    // setup the producer (for outgoing events)
    let producer = Arc::new(events::producers::EventProducer::new(
        messaging_client.clone(),
    ));

    // todo: monitor for bottlenecks and benchmarks the service and use deadpool-redis if there is evidence of bottleneck with single connection
    // setup Redis connection for state management
    let client = redis::Client::open(redis_url)?;
    let con = client.get_multiplexed_async_connection().await?;

    // setup the matcher service (business logic)
    let matcher_service = Arc::new(matcher::service::MatcherService::new(
        producer.clone(),
        con.clone(),
    ));

    // setup the consumers (incoming events)
    let consumers = events::consumers::Consumers::new(messaging_client.clone());
    consumers.register_all(matcher_service.clone()).await;

    // Wait here so the service keeps running until interrupted (e.g., with Ctrl+C).
    // Using `tokio::signal::ctrl_c().await` allows graceful shutdown on user interrupt,
    // while `futures::future::pending().await` would block forever without handling signals.
    tokio::signal::ctrl_c().await?;
    // futures::future::pending::<()>().await;

    Ok(())
}
