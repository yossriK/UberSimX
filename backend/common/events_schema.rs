// This file will hold all event schemas shared across the project.
// Define your event structs and enums here.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, Serialize)]
pub struct DriverAvailabilityChangedEvent {
    pub driver_id: Uuid,
    pub driver_available: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateRideRequest {
    pub ride_id: Uuid,
    pub rider_id: Uuid,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub created_at: DateTime<Utc>,
}
