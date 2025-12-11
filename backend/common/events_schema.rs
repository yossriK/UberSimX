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
pub struct RideRequestedEvent {
    pub ride_id: Uuid,
    pub rider_id: Uuid,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DriverAssignedRideEvent {
    pub ride_id: Uuid,
    pub driver_id: Uuid,
    pub assigned_at: DateTime<Utc>,
    pub pickup_lat: f64,
    pub pickup_lng: f64,
    pub dropoff_lat: f64,
    pub dropoff_lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoDriversAvailableEvent {
    pub ride_id: Uuid,
    pub rider_id: Uuid,
    pub requested_at: DateTime<Utc>,
    pub reason: Option<String>, // e.g., "timeout", "all drivers busy", etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DriverAcceptedRideEvent {
    pub ride_id: Uuid,
    pub driver_id: Uuid,
    pub accepted_at: DateTime<Utc>,
    pub estimated_pickup_time_minutes: u32,
    // todo how are we showing the rider service how to get drivier locaton updates
    // pub driver_location_lat: f64,
    // pub driver_location_lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DriverRejectedRideEvent {
    pub ride_id: Uuid,
    pub driver_id: Uuid,
}