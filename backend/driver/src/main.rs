use anyhow::Result;
use futures_util::stream::StreamExt;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tokio::task;
use ubersimx_messaging::{messagingclient::MessagingClient, Messaging};

use crate::{
    api::router::{create_router, AppState},
    repository::{driver_repository::PgDriverRepository, vehicle_repository::PgVehicleRepository},
};

mod models;
// TODO: not supposed to use unwrap I know, but I was experimenting to get a skeleton mvp quick
pub mod repository {
    pub mod driver_repository;
    pub mod vehicle_repository;
    // DriverLocation usually not persisted in a database for high frequency updates. will be in memrory or cache
    // PostgreSQL + PostGIS extension OR Redis + Geo commands

    // DriverAvailabilityEvent don't need to be stored in a database, they are transient messages, that exist as events
    // in the messaging system

    // DriverStatus could be peristed, but for simulation in memroy is fine.
}

pub mod api {
    pub mod driver;
    pub mod router;
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_filename("settings.env").ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|e| anyhow::anyhow!("DATABASE_URL must be set in .env: {}", e))?;

    let messaging_url = env::var("MESSAGING_URL")
        .map_err(|e| anyhow::anyhow!("MESSAGING_URL must be set in .env: {}", e))?;

    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    // Create a connection pool
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?,
    );

    let driver_repo = Arc::new(PgDriverRepository::new(pool.clone()));
    let vehicle_repo = Arc::new(PgVehicleRepository::new(pool.clone()));

    // Connect to your messaging service
    let client = Arc::new(MessagingClient::connect(&messaging_url).await.unwrap());

    // can also have factory function to create AppState that takes pool and creates repos inside
    let state = AppState {
        driver_repo,
        vehicle_repo,
        messaging_client: client.clone(),
    };
    let app = create_router(state);

    // Spawn a Tokio task to subscribe to "driver.signup"
    let handle = task::spawn(async move {
        let mut subscription = client
            .subscribe(String::from("driver.signup"))
            .await
            .unwrap();
        while let Some(msg) = subscription.next().await {
            println!(
                "Received: {:?}",
                std::str::from_utf8(&msg.unwrap().data).unwrap()
            );
        }
    });

    // Start server
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    // Run both the server and message handler concurrently using tokio::select!
    // This ensures both tasks run simultaneously and we can detect failures in either:
    // - If the HTTP server crashes, we'll know about it
    // - If the message subscription task panics, we'll catch it
    // Without select!, axum::serve() would block forever and we'd never await the handle,
    // causing silent failures in the message handler that we couldn't detect or recover from
    tokio::select! {
        result = axum::serve(listener, app) => {
            if let Err(e) = result {
                eprintln!("Server error: {}", e);
            }
        }
        result = handle => {
            if let Err(e) = result {
                eprintln!("Message handler error: {}", e);
            }
        }
    }

    Ok(())
}
