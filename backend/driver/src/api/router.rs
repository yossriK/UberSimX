use crate::api::driver;
use crate::repository::{driver_repository::DriverRepository, vehicle_repository};
use axum::{
    routing::{get, post},
    Router,
};
use ubersimx_messaging::messagingclient::MessagingClient;

use crate::repository::vehicle_repository::VehicleRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState<D, C> {
    pub driver_repo: Arc<D>,
    pub vehicle_repo: Arc<C>,
    pub messaging_client: Arc<MessagingClient>,
}

pub fn create_router<D, C>(state: AppState<D, C>) -> Router
where
    D: DriverRepository + Send + Sync + Clone + 'static,
    C: VehicleRepository + Send + Sync + Clone + 'static,
{
    Router::new()
        // Driver routes
        .route("/drivers", post(driver::create_driver))
        // .route("/drivers", get(driver::list_drivers::<D>))
        // .route("/drivers/:id", get(driver::get_driver::<D>))
        // Car routes
        // .route("/vehicles", post(vehicle_repository::create_vehicle::<D, C>))
        // .route("/vehicles", get(car::list_vehicles::<D, C>))
        // .route("/vehicles/:id", get(car::get_car::<D, C>))
        // hook the state
        // there is .layer that allows to attach different bits of state separately, like DBpool, metrics, feature flag store etc
        .with_state(state)
}
