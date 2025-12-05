use common::events_schema::DriverAvailabilityChangedEvent;
use common::redis_key_helpers::driver_metadata_namespace;
use common::redis_namespaces::DRIVER_LAST_SEEN_KEY;
use common::redis_namespaces::DRIVER_LOCATION_NAMESPACE;
use common::subjects::DRIVER_AVAILABILITY_SUBJECT;
use redis::AsyncTypedCommands;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
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
use crate::repository::driver_status_repository::DriverStatusRepository;
use crate::repository::driver_status_repository::RideStatus;
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

#[derive(Deserialize)]
pub struct DriverLocationUpdateRequest {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize)]
pub struct DriverStatusUpdateRequest {
    pub driver_available: bool,
}

pub async fn create_driver<D, C>(
    State(state): State<AppState<D, C>>,
    Json(payload): Json<CreateDriverRequest>,
) -> Result<Json<DriverResponse>, StatusCode>
where
    D: DriverRepository + Send + Sync + Clone + 'static,
    C: DriverStatusRepository + Send + Sync + Clone + 'static,
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

    let event = DriverAvailabilityChangedEvent {
        driver_id: driver.id,
        driver_available: true,
    };

    let payload = match serde_json::to_vec(&event) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to serialize DriverAvailabilityChangedEvent: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Send NATS event for DriverAvailabilityChangedEvent
    if let Err(e) = state
        .messaging_client
        .publish(DRIVER_AVAILABILITY_SUBJECT.to_string(), payload)
        .await
    {
        // should be sufficient to just print as long as the database creation was successful
        eprintln!("Failed to publish {DRIVER_AVAILABILITY_SUBJECT} : {}", e);
    }
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

pub async fn update_driver_location<D, C>(
    State(state): State<AppState<D, C>>,
    Path(driver_id): Path<Uuid>,
    Json(payload): Json<DriverLocationUpdateRequest>,
) -> Result<StatusCode, StatusCode>
where
    D: DriverRepository + Send + Sync + Clone + 'static,
    C: DriverStatusRepository + Send + Sync + Clone + 'static,
{
    // todo check if the driver exists before updating location

    // Here you would typically update the driver's location in the database
    // For simulation purposes, we'll just print it out

    println!(
        "Updating location for driver {}: lat={}, lon={}",
        driver_id, payload.latitude, payload.longitude
    );

    // Update location in Redis using GEOADD
    state
        .redis_con
        .lock()
        .await
        .geo_add(
            DRIVER_LOCATION_NAMESPACE,
            (payload.longitude, payload.latitude, driver_id.to_string()),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    // update the timestamp of the last location update
    let timestamp = chrono::Utc::now().timestamp_millis();
    
    state.redis_con.lock().await.hset(
        driver_metadata_namespace(driver_id),
        DRIVER_LAST_SEEN_KEY,
        timestamp,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    // todo this code to be removed after testing
    let pos: Option<Vec<Option<(f64, f64)>>> = state
        .redis_con
        .lock()
        .await
        .geo_pos("drivers:locations", &[driver_id.to_string()])
        .await
        .ok()
        .map(|vec| {
            vec.into_iter()
                .map(|opt_coord| opt_coord.map(|coord| (coord.longitude, coord.latitude)))
                .collect()
        });

    if let Some(Some((lon, lat))) = pos.and_then(|v| v.into_iter().next()) {
        println!(
            "Driver {} location updated to: lat={}, lon={}",
            driver_id, lat, lon
        );
    } else {
        println!(
            "Failed to update location for driver {}: not found in Redis",
            driver_id
        );
    }
    Ok(StatusCode::OK)
}

pub async fn update_driver_status<D, C>(
    State(state): State<AppState<D, C>>,
    Path(driver_id): Path<Uuid>,
    Json(driver_status_request): Json<DriverStatusUpdateRequest>,
) -> Result<StatusCode, StatusCode>
where
    D: DriverRepository + Send + Sync + Clone + 'static,
    C: DriverStatusRepository + Send + Sync + Clone + 'static,
{
    // Try to patch (update) the status first
    let patch_result = state
        .driver_status_repo
        .patch_status(
            driver_id,
            Some(driver_status_request.driver_available),
            None,
            None,
        )
        .await;
    let event = DriverAvailabilityChangedEvent {
        driver_id: driver_id,
        driver_available: driver_status_request.driver_available,
    };

    let payload = match serde_json::to_vec(&event) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to serialize DriverAvailabilityChangedEvent: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match patch_result {
        Ok(_) => {
            // Send NATS event for DriverAvailabilityChangedEvent
            if let Err(e) = state
                .messaging_client
                .publish(DRIVER_AVAILABILITY_SUBJECT.to_string(), payload)
                .await
            {
                eprintln!("Failed to publish {DRIVER_AVAILABILITY_SUBJECT} : {}", e);
            }
            Ok(StatusCode::OK)
        }
        Err(_) => {
            // If update failed (e.g., no record), try to create
            let new_status = crate::repository::driver_status_repository::DriverStatus {
                driver_id,
                driver_available: driver_status_request.driver_available,
                ride_status: RideStatus::None,
                current_trip_id: None,
                status_updated_at: chrono::Utc::now(),
            };
            let result = state
                .driver_status_repo
                .create_status(&new_status)
                .await
                .map(|_| StatusCode::CREATED)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

            if let Err(e) = state
                .messaging_client
                .publish(DRIVER_AVAILABILITY_SUBJECT.to_string(), payload)
                .await
            {
                eprintln!("Failed to publish {DRIVER_AVAILABILITY_SUBJECT} : {}", e);
            }
            result
        }
    }
}
