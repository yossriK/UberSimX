use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;





use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};

use crate::repository::driver_repository::DriverRepository;
use crate::models::Driver;
use super::{CreateDriverRequest, DriverResponse};
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

pub async fn create_driver<R: DriverRepository>(
    State(repo): State<Arc<R>>,
    Json(payload): Json<CreateDriverRequest>,
) -> Result<Json<DriverResponse>, StatusCode> {
    let driver = Driver {
        id: Uuid::new_v4(),
        name: payload.name,
        car_id: payload.car_id,
    };

    repo.create_driver(&driver).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    let drivers = repo.list_drivers().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let resp: Vec<DriverResponse> = drivers.into_iter()
        .map(|d| DriverResponse { id: d.id, name: d.name, car_id: d.car_id })
        .collect();
    Ok(Json(resp))
}
