/// Handles WebSocket upgrade requests for driver connections.
/// 
/// Extracts the client ID from query parameters (or generates a new UUID if not provided),
/// and upgrades the HTTP connection to a WebSocket. Once upgraded, the socket is passed
/// to `ws_on_upgrade` along with the WebSocket hub and location update service for
/// managing real-time driver location updates and connection lifecycle.
use axum::{
    extract::{Query, State, WebSocketUpgrade},
    response::Response,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    api::router::AppState,
    infra::{
        repository::{
            driver_repository::DriverRepository, driver_status_repository::DriverStatusRepository,
        },
        ws::{connections::ws_on_upgrade},
    },
};

pub async fn ws_handler<D, C>(
    State(state): State<AppState<D, C>>,
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
) -> Response
where
    D: DriverRepository + Send + Sync + Clone + 'static,
    C: DriverStatusRepository + Send + Sync + Clone + 'static,
{
    let client_id = params
        .get("client_id")
        .and_then(|id| Uuid::parse_str(id).ok())
        .unwrap_or_else(Uuid::new_v4);

    let hub = state.ws_hub.clone();
    let location_update_service = state.location_update_service.clone();
    
    ws.on_upgrade(move |socket| {
        ws_on_upgrade(
            socket,
            hub.clone(),
            location_update_service.clone(),
            client_id,
        )
    })
}
