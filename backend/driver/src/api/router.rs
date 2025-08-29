use axum::{Router, routing::{get, post}};
use crate::repository::{driver_repository::DriverRepository, vehicle_repository};
use crate::api::driver;


use std::sync::Arc;
use crate::repository::{vehicle_repository::VehicleRepository};

#[derive(Clone)]
pub struct AppState<D, C> {
    pub driver_repo: Arc<D>,
    pub vehicle_repo: Arc<C>,
}

pub fn create_router<D, C>() -> Router
where
    D: DriverRepository + 'static,
    C: VehicleRepository + 'static,
{
    Router::new()
        // Driver routes
        .route("/drivers", post(driver::create_driver::<D>))
        .route("/drivers", get(driver::list_drivers::<D>))
        .route("/drivers/:id", get(driver::get_driver::<D>))
        // Car routes
        // .route("/vehicles", post(vehicle_repository::create_vehicle::<D, C>))
        // .route("/vehicles", get(car::list_vehicles::<D, C>))
        // .route("/vehicles/:id", get(car::get_car::<D, C>))

        // hook the state
        // .with_state(AppState {})
}
