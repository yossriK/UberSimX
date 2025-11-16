use crate::models::{CreateRideRequest, CreateRiderRequest, Ride, Rider};
use crate::repository::riders_repository::RidersRepository;
use crate::repository::rides_repository::RidesRepository;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub struct AppState {
    pub riders_repo: Arc<RidersRepository>,
    pub rides_repo: Arc<RidesRepository>,
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
) -> Result<Json<Ride>, axum::http::StatusCode> {
    let request = CreateRideRequest {
        rider_id: payload.rider_id,
        origin_lat: payload.origin_lat,
        origin_lng: payload.origin_lng,
        destination_lat: payload.destination_lat,
        destination_lng: payload.destination_lng,
    };

    match state.rides_repo.create_ride(request).await {
        Ok(ride) => Ok(Json(ride)),
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
