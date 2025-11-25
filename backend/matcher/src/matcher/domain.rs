use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::events::schema::RideRequestedEvent;

/// Represents the state of a driver in the system.
#[derive(Debug, Clone)]
pub struct DriverState {
    pub driver_id: String,
    pub lat: f64,
    pub lon: f64,
    pub available: bool,
}

/// Represents a ride record in the matcher domain.
#[derive(Debug, Clone)]
pub struct RideRecord {
    pub ride_id: Uuid,
    pub rider_id: Uuid,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub created_at: DateTime<Utc>,
    pub status: RideStatus,
}

/// Status of a ride in the matcher domain.
#[derive(Debug, Clone)]
pub enum RideStatus {
    Pending,
    Matched,
    Expired,
}

impl From<RideRequestedEvent> for RideRecord {
    fn from(event: RideRequestedEvent) -> Self {
        RideRecord {
            ride_id: event.ride_id,
            rider_id: event.rider_id,
            origin_lat: event.origin_lat,
            origin_lng: event.origin_lng,
            destination_lat: event.destination_lat,
            destination_lng: event.destination_lng,
            created_at: event.created_at,
            // the status is hardcoded to pending here, as this conversion is specifically for new ride requests
            // so it shouldn't be a problem (I hope :) )
            status: RideStatus::Pending,
        }
    }
}