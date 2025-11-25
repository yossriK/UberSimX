use crate::models::{CreateRideRequest, CreateRiderRequest, Ride, Rider};
use crate::repository::riders_repository::RidersRepository;
use crate::repository::rides_repository::RidesRepository;
use axum::{routing::post, Json, Router};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use ubersimx_messaging::{messagingclient::MessagingClient, Messaging, subjects::RIDER_REQUESTED_SUBJECT};
use uuid::Uuid;

pub struct AppState {
    pub riders_repo: Arc<RidersRepository>,
    pub rides_repo: Arc<RidesRepository>,
    pub messaging_client: Arc<MessagingClient>,
}

#[derive(Deserialize)]
struct CreateRider {
    name: String,
}

async fn create_rider(
    state: axum::extract::State<Arc<AppState>>,
    Json(payload): Json<CreateRider>,
) -> Result<Json<Rider>, axum::http::StatusCode> {
    let request = CreateRiderRequest { name: payload.name };

    match state.riders_repo.create_rider(request).await {
        Ok(rider) => Ok(Json(rider)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct RequestRide {
    rider_id: Uuid,
    origin_lat: f64,
    origin_lng: f64,
    destination_lat: f64,
    destination_lng: f64,
}

async fn request_ride(
    state: axum::extract::State<Arc<AppState>>,
    Json(payload): Json<RequestRide>,
) -> Result<(), axum::http::StatusCode> {

    // todo: validate rider exists

    let request = CreateRideRequest {
        ride_id: Uuid::new_v4(),
        rider_id: payload.rider_id,
        origin_lat: payload.origin_lat,
        origin_lng: payload.origin_lng,
        destination_lat: payload.destination_lat,
        destination_lng: payload.destination_lng,
        created_at: Utc::now(),
    };

    // You should send the event after calling the repository, for these important reasons:
    // Data consistency - Only send events for rides that were successfully persisted to the database
    // Reliability - If the database operation fails, you don't want to send misleading events
    // Event ordering - Events should reflect the actual state changes that occurred
    // Error handling - You can handle database errors without worrying about "orphaned" events

    match state.rides_repo.create_ride(request.clone()).await {
        Ok(_) => {
            // 2. Then, send event (with the actual ride data including generated ID)
            let ride_request_data = serde_json::to_vec(&request).unwrap_or_default();
            if let Err(_) = state
                .messaging_client
                .publish(RIDER_REQUESTED_SUBJECT.to_string(), ride_request_data)
                .await
            {
                // todo: proper clean up, like delete the db transaction or retry logic could be implemented here

                // Log the error but don't fail the request since ride is already created

                eprintln!("Failed to send ride requested event");
                return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
            }

            Ok(())
        }

        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

use axum::extract::Path;
use axum::http::StatusCode;

async fn get_rider(
    state: axum::extract::State<Arc<AppState>>,
    Path(rider_id): Path<Uuid>,
) -> Result<Json<Rider>, StatusCode> {
    match state.riders_repo.get_rider_by_id(rider_id).await {
        Ok(Some(rider)) => Ok(Json(rider)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/riders", post(create_rider))
        .route("/riders/{id}", axum::routing::get(get_rider))
        .route("/rides", post(request_ride))
        .with_state(state)
}
