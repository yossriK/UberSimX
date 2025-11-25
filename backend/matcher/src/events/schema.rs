use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
// defines all event types (RideRequested, DriverLocationUpdated, etc.)

#[derive(Debug, Serialize, Deserialize)]
pub struct RideRequestedEvent {
    pub ride_id: Uuid,
    pub rider_id: Uuid,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchProposedEvent {
    pub ride_id: String,
    pub driver_id: String,
    pub rider_id: String,
}
