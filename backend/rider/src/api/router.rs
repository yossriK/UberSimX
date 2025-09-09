use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Rider {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RideRequest {
    pub user_id: Uuid,
    pub pickup: String,
    pub dropoff: String,
}

pub struct AppState {
    pub riders: Mutex<HashMap<Uuid, Rider>>,
    pub rides: Mutex<Vec<RideRequest>>,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_rider(
    state: axum::extract::State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Json<Rider> {
    let rider = Rider {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
    };
    state.riders.lock().unwrap().insert(rider.id, rider.clone());
    Json(rider)
}

#[derive(Deserialize)]
struct RequestRide {
    user_id: Uuid,
    pickup: String,
    dropoff: String,
}

async fn request_ride(
    state: axum::extract::State<Arc<AppState>>,
    Json(payload): Json<RequestRide>,
) -> Json<RideRequest> {
    let ride = RideRequest {
        user_id: payload.user_id,
        pickup: payload.pickup,
        dropoff: payload.dropoff,
    };
    state.rides.lock().unwrap().push(ride.clone());
    Json(ride)
}

use axum::extract::Path;
use axum::http::StatusCode;

async fn get_rider(
    state: axum::extract::State<Arc<AppState>>,
    Path(rider_id): Path<Uuid>,
) -> Result<Json<Rider>, StatusCode> {
    let riders = state.riders.lock().unwrap();
    if let Some(rider) = riders.get(&rider_id) {
        Ok(Json(rider.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/riders", post(create_rider))
        .route("/riders/{id}", axum::routing::get(get_rider))
        .route("/rides", post(request_ride))
        .with_state(state)
}
