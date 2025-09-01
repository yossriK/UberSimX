use serde::Deserialize;
use serde::Serialize;
use ubersimx_messaging::messagingclient;
use ubersimx_messaging::messagingclient::MessagingClient;
use ubersimx_messaging::Messaging;
use uuid::Uuid;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::api::router::AppState;
use crate::models::Driver;
use crate::repository::driver_repository::DriverRepository;
use crate::repository::vehicle_repository::VehicleRepository;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateDriverRequest {
    pub name: String,
    pub car_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct DriverResponse {
    pub id: Uuid,
    pub name: String,
    pub car_id: Option<Uuid>,
}

pub async fn create_driver<D, C>(
    State(state): State<AppState<D, C>>,
    Json(payload): Json<CreateDriverRequest>,
) -> Result<Json<DriverResponse>, StatusCode>
where
    D: DriverRepository + Send + Sync + Clone + 'static,
    C: VehicleRepository + Send + Sync + Clone + 'static,
{
    let repo = state.driver_repo.clone();

    let driver = Driver {
        id: Uuid::new_v4(),
        name: payload.name,
        car_id: payload.car_id,
        license_number: None,
        rating: None,
    };

    repo.create_driver(&driver)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // for the fun of it let me hook the publisher and just publish for fun and see if itll be received.
    state
        .messaging_client
        .publish(
            String::from("driver.signup"),
            "Ride started".as_bytes().to_vec(),
        )
        .await
        .unwrap();

    Ok(Json(DriverResponse {
        id: driver.id,
        name: driver.name,
        car_id: driver.car_id,
    }))
}

pub async fn get_driver<R: DriverRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<Uuid>,
) -> Result<Json<DriverResponse>, StatusCode> {
    match repo.get_driver(id).await {
        Ok(Some(driver)) => Ok(Json(DriverResponse {
            id: driver.id,
            name: driver.name,
            car_id: driver.car_id,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_drivers<R: DriverRepository>(
    State(repo): State<Arc<R>>,
) -> Result<Json<Vec<DriverResponse>>, StatusCode> {
    let drivers = repo
        .list_drivers()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let resp: Vec<DriverResponse> = drivers
        .into_iter()
        .map(|d| DriverResponse {
            id: d.id,
            name: d.name,
            car_id: d.car_id,
        })
        .collect();
    Ok(Json(resp))
}
