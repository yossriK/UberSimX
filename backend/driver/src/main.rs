use anyhow::Result;
use futures_util::stream::StreamExt;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tokio::task;
use ubersimx_messaging::{messagingclient::MessagingClient, Messaging};

use crate::api::router::{create_router, AppState};
use crate::infra::repository::driver_repository::PgDriverRepository;
use crate::infra::repository::driver_status_repository::PgDriverStatusRepository;
use crate::infra::repository::vehicle_repository::PgVehicleRepository;

mod models;
pub mod infra {
    pub mod repository {
        pub mod driver_repository;
        pub mod driver_status_repository;
        pub mod vehicle_repository;
    }
}
// TODO: not supposed to use unwrap I know, but I was experimenting to get a skeleton mvp quick
// repository modules moved to infra/repository

pub mod api {
    pub mod driver;
    pub mod router;
}

mod service {
    pub mod redis_cleanup;
    pub mod ride_lifecycle;
    pub mod eta_service;
}

pub mod events {
    pub mod handlers;
    pub mod publisher;
    pub mod schemas;
    pub mod subscribers;
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_filename("settings.env").ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|e| anyhow::anyhow!("DATABASE_URL must be set in .env: {}", e))?;

    let messaging_url = env::var("MESSAGING_URL")
        .map_err(|e| anyhow::anyhow!("MESSAGING_URL must be set in .env: {}", e))?;

    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3001".to_string());

    let redis_url = env::var("REDIS_URL")
        .map_err(|e| anyhow::anyhow!("REDIS_URL must be set in .env: {}", e))?;

    // Create a connection pool
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?,
    );

    let driver_repo = Arc::new(PgDriverRepository::new(pool.clone()));
    let vehicle_repo = Arc::new(PgVehicleRepository::new(pool.clone()));
    let driver_status_repo = Arc::new(PgDriverStatusRepository::new(pool.clone()));

    // Connect to your messaging service
    let messaging_client = Arc::new(MessagingClient::connect(&messaging_url).await.unwrap());

    // setup Redis connection for live state management (e.g., driver locations) vs PostgreSQL for persistent storage
    let redis_client = redis::Client::open(redis_url)?;
    let con = redis_client.get_multiplexed_async_connection().await?;

    // setup the producer (for outgoing events)
    let event_publisher = Arc::new(events::publisher::EventPublisher::new(
        messaging_client.clone(),
    ));

    // Create Usecases
    let ride_lifecycle_service = Arc::new(service::ride_lifecycle::RideLifeCycleService {
        driver_status_repo: driver_status_repo.clone(),
        producer: event_publisher.clone(),
        redis_con: Arc::new(tokio::sync::Mutex::new(con.clone())),
    });

    // setup the consumers (incoming events)
    let event_subscribers = events::subscribers::Subscribers::new(messaging_client.clone());
    event_subscribers
        .register_ride_evnets_consumers(ride_lifecycle_service.clone())
        .await;

    // can also have factory function to create AppState that takes pool and creates repos inside
    // todo clean up this to take usecases instead of infra repos directly
    let state = AppState {
        driver_repo,
        driver_status_repo,
        messaging_client: messaging_client.clone(),
        redis_con: Arc::new(tokio::sync::Mutex::new(con)),
        ride_lifecycle_service: ride_lifecycle_service.clone(),
    };
    let app = create_router(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    axum::serve(listener, app)
        .await
        .map_err(|_| anyhow::anyhow!("Axum Server error"))?;

    Ok(())
}
