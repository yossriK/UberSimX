use std::sync::Arc;

use uuid::Uuid;

pub trait LocationUpdate {
    async fn handle_location_update(&self, driver_id: Uuid, latitude: f64, longitude: f64);
}

/// Service responsible for handling driver location updates.
pub struct LocationUpdateService {
    pub redis_con: Arc<tokio::sync::Mutex<redis::aio::MultiplexedConnection>>,
}

impl LocationUpdate for LocationUpdateService {
    /// Handle a location update for a driver.
    async fn handle_location_update(&self, driver_id: Uuid, latitude: f64, longitude: f64) {
        // TODO validate inputs
        // TODO: Implement logic to update location in Redis, DB, or publish event
        println!(
            "Location update: driver_id={}, lat={}, lng={}",
            driver_id, latitude, longitude
        );
    }
}
