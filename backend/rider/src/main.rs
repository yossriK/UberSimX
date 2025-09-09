pub mod api {
    pub(crate) mod router;
}
use api::router::{create_router, AppState};

use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let state = Arc::new(AppState {
        riders: std::sync::Mutex::new(std::collections::HashMap::new()),
        rides: std::sync::Mutex::new(Vec::new()),
    });

    let app = create_router(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
