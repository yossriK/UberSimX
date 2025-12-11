// all Events are shared among services so I put them in the Common lib crate. However this would be a good
// place to map between MessagingClient Events and internal Driver service events if needed.

use common::events_schema::DriverAssignedRideEvent;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// DTO for DriverAssignedRideEvent payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverAssignedRideDto {
    pub driver_id: Uuid,
    pub ride_id: Uuid,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub pickup_lat: f64,
    pub pickup_lng: f64,
    pub dropoff_lat: f64,
    pub dropoff_lng: f64,
}

/// Mapper from DriverAssignedRideEvent to DriverAssignedRideDto
impl From<&DriverAssignedRideEvent> for DriverAssignedRideDto {
    fn from(event: &DriverAssignedRideEvent) -> Self {
        Self {
            driver_id: event.driver_id,
            ride_id: event.ride_id,
            assigned_at: event.assigned_at,
            pickup_lat: event.pickup_lat,
            pickup_lng: event.pickup_lng,
            dropoff_lat: event.dropoff_lat,
            dropoff_lng: event.dropoff_lng,
        }
    }
}
