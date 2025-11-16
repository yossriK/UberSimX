use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use tokio::task;
use ubersimx_messaging::{messagingclient::MessagingClient, Messaging};
use std::{env, sync::Arc};
use futures_util::stream::StreamExt;

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

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?,
    );

    let driver_repo = Arc::new(PgDriverRepository::new(pool.clone()));
    let vehicle_repo = Arc::new(PgVehicleRepository::new(pool.clone()));

    // Connect to your messaging service
    let client = Arc::new(MessagingClient::connect("localhost:4222").await.unwrap());

    // can also have factory function to create AppState that takes pool and creates repos inside
        let state = AppState {
        driver_repo,
        vehicle_repo,
        messaging_client: client.clone(),
    };
    let app = create_router(state);




    // Spawn a Tokio task to subscribe to "events.ride"
    let handle = task::spawn(async move {
        let mut subscription = client.subscribe(String::from("driver.signup")).await.unwrap();
        while let Some(msg) = subscription.next().await {
            println!("Received: {:?}", std::str::from_utf8(&msg.unwrap().data).unwrap());
        }
    });

    // Start server
     // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();



    Ok(())
}
