// Models module - Central location for all data structures
//
// This module contains all domain models and DTOs used across the application.
// Benefits of centralizing models here:
// 1. Single source of truth - One definition used by repositories, API handlers, and services
// 2. Prevents circular dependencies - Models don't depend on business logic layers
// 3. Easier maintenance - Schema changes only need to be made in one place
// 4. Better testability - Models can be tested independently
// 5. Consistency - Same struct definitions across all application layers
// 6. Reusability - Models can be shared between multiple repositories or services

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rider {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ride {
    pub id: Uuid,
    pub rider_id: Uuid,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub status: String,
    pub driver_id: Option<Uuid>,
    pub match_time: Option<DateTime<Utc>>,
    pub pickup_time: Option<DateTime<Utc>>,
    pub dropoff_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRiderRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRideRequest {
    pub rider_id: Uuid,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
}
