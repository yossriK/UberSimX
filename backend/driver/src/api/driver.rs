use common::events_schema::DriverAvailabilityChangedEvent;
use common::redis_key_helpers::driver_state_namespace;
use common::redis_namespaces::DRIVER_AVAILABILITY_FIELD;
use common::redis_namespaces::DRIVER_AVAILABILITY_REASON_FIELD;
use common::redis_namespaces::DRIVER_LAST_AVAILABILITY_UPDATE_FIELD;
use common::redis_namespaces::DRIVER_LAST_LOCATION_UPDATE_FIELD;
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
use crate::models::AvailabilityReason;
use crate::models::Driver;
use crate::models::DriverRedisState;
use crate::models::DriverStatus;
use crate::models::RideStatus;
use crate::infra::repository::driver_repository::DriverRepository;
use crate::infra::repository::driver_status_repository::DriverStatusRepository;
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

    // Set driver location and availability using a Redis pipeline (atomic MULTI/EXEC).
    // This issues the commands in one network round trip and executes them
    // inside MULTI/EXEC so they are applied atomically on the server.

    let timestamp = chrono::Utc::now().timestamp_millis();

    // Build pipeline
    let mut pipe = redis::pipe();
    pipe.atomic()
        .geo_add(
            DRIVER_LOCATION_NAMESPACE,
            (payload.longitude, payload.latitude, driver_id.to_string()),
        )
        .ignore()
        .hset(
            driver_state_namespace(driver_id),
            DRIVER_LAST_LOCATION_UPDATE_FIELD,
            timestamp,
        )
        .expire(driver_state_namespace(driver_id), 90);

    let mut con = state.redis_con.lock().await;
    // Execute pipeline
    pipe.query_async::<()>(&mut *con)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// todo this needs cleanup as we got lots of nesting and repeated code
pub async fn update_driver_status<D, C>(
    State(state): State<AppState<D, C>>,
    Path(driver_id): Path<Uuid>,
    Json(driver_status_request): Json<DriverStatusUpdateRequest>,
) -> Result<StatusCode, StatusCode>
where
    D: DriverRepository + Send + Sync + Clone + 'static,
    C: DriverStatusRepository + Send + Sync + Clone + 'static,
{
    // before updating here we need to check if the driver is in ride or not
    // becuase the client app might send availability updates while in ride.
    // We do assume that Redis at this point should be in sync with the database state
    // In a real-world scenario, we might want to have more robust consistency checks.
    // we are constantly updating redis as evnets in, and the cron job also keeps redis
    // in a sync state.

    let key = driver_state_namespace(driver_id);

    let mut con = state.redis_con.lock().await;
    let driver_state_map: std::collections::HashMap<String, String> = match con.hgetall(&key).await
    {
        Ok(map) => map,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    drop(con); // release the lock early as it will be reacquired later

    // if there is no record in redis, that means the record was deleted, or expired due to inactivity
    // which means driver is offline. In this case we can just patch the database directly to offline.
    // if the driver is coming online we will also set the redis key again.
    if driver_state_map.is_empty() {
        // patch driver in the database directly if there is no redis key.
        let patch_result = state
            .driver_status_repo
            .patch_status(
                driver_id,
                Some(driver_status_request.driver_available),
                None,
                None,
            )
            .await;
        // Only write to Redis if the driver is coming online
        if driver_status_request.driver_available {
            let mut con = state.redis_con.lock().await;
            let mut pipe = redis::pipe();
            pipe.atomic()
                .hset(&key, DRIVER_AVAILABILITY_FIELD, true)
                .hset(
                    &key,
                    DRIVER_AVAILABILITY_REASON_FIELD,
                    AvailabilityReason::Available.to_string(),
                )
                .hset(
                    &key,
                    DRIVER_LAST_AVAILABILITY_UPDATE_FIELD,
                    chrono::Utc::now().timestamp(),
                )
                .expire(&key, 90);
            pipe.query_async::<()>(&mut *con)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        match patch_result {
            Ok(_) => {
                return Ok(StatusCode::OK);
            }
            Err(_) => {
                // If patching the driver status in the database fails, return an error.
                // If update failed (e.g., no record), try to create
                let new_status = DriverStatus {
                    driver_id,
                    driver_available: driver_status_request.driver_available,
                    ride_status: RideStatus::None,
                    current_trip_id: None,
                    status_updated_at: chrono::Utc::now(),
                };
                let _ = state
                    .driver_status_repo
                    .create_status(&new_status)
                    .await
                    .map(|_| StatusCode::CREATED)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    // Convert String HashMap to JSON string
    // I mean we could potentially assume that key doesn't exist and override it but I will not have extra error recover here just return errro
    let json =
        serde_json::to_string(&driver_state_map).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Deserialize to struct
    let redis_driver_state: DriverRedisState =
        serde_json::from_str(&json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let availability =
        crate::models::compute_availability(chrono::Utc::now().timestamp(), &redis_driver_state);

    if !availability.available
        && driver_status_request.driver_available
        && availability.reason == crate::models::AvailabilityReason::InRide
    {
        // trying to set available when not allowed. if user was offline or stale location this means they are coming back online.
        // but if they are in a ride we will not change their status atm
        return Err(StatusCode::OK); // no-op but return OK
    }

    let patch_result = state
        .driver_status_repo
        .patch_status(
            driver_id,
            Some(driver_status_request.driver_available),
            None,
            None,
        )
        .await;



// there are no subscribers for this evnet but will keep the code for reference
    /*  let event = DriverAvailabilityChangedEvent {
            driver_id,
            driver_available: driver_status_request.driver_available,
        };

        let payload = match serde_json::to_vec(&event) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to serialize DriverAvailabilityChangedEvent: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    */
    let mut con = state.redis_con.lock().await;
    let mut pipe = redis::pipe();
    pipe.atomic()
        .hset(&key, DRIVER_AVAILABILITY_FIELD, true)
        .hset(
            &key,
            DRIVER_AVAILABILITY_REASON_FIELD,
            if driver_status_request.driver_available {
                AvailabilityReason::Available.to_string()
            } else {
                AvailabilityReason::OfflineToggle.to_string()
            },
        )
        .hset(
            &key,
            DRIVER_LAST_AVAILABILITY_UPDATE_FIELD,
            chrono::Utc::now().timestamp(),
        )
        .expire(&key, 90);
    pipe.query_async::<()>(&mut *con)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match patch_result {
        Ok(_) => {
            //todo: there is no more subscribers to this but will keep it just for reference Send NATS event for DriverAvailabilityChangedEvent
            // if let Err(e) = state
            //     .messaging_client
            //     .publish(DRIVER_AVAILABILITY_SUBJECT.to_string(), payload)
            //     .await
            // {
            //     eprintln!("Failed to publish {DRIVER_AVAILABILITY_SUBJECT} : {}", e);
            // }

            Ok(StatusCode::OK)
        }
        Err(_) => {
            // If update failed (e.g., no record), try to create
            let new_status = DriverStatus {
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

            // if let Err(e) = state
            //     .messaging_client
            //     .publish(DRIVER_AVAILABILITY_SUBJECT.to_string(), payload)
            //     .await
            // {
            //     eprintln!("Failed to publish {DRIVER_AVAILABILITY_SUBJECT} : {}", e);
            // }
            result
        }
    }
}
