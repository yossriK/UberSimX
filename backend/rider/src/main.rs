pub mod api {
    pub(crate) mod router;
}

pub mod models;

pub mod repository {
    pub mod riders_repository;
    pub mod rides_repository;
}

use api::router::{create_router, AppState};
use repository::riders_repository::RidersRepository;
use repository::rides_repository::RidesRepository;
use sqlx::postgres::PgPoolOptions;

use anyhow::Result;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_filename("settings.env").ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|e| anyhow::anyhow!("DATABASE_URL must be set in .env: {}", e))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let riders_repo = Arc::new(RidersRepository::new(pool.clone()));
    let rides_repo = Arc::new(RidesRepository::new(pool.clone()));

    let state = Arc::new(AppState {
        riders_repo,
        rides_repo,
    });

    let app = create_router(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
