use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Driver {
    pub id: Uuid,
    pub name: String,
    pub license_number: Option<String>, // might not be known at time of creation
    pub rating: Option<f32>,            // not known at time of creation
    pub car_id: Option<Uuid>,           // might not be assigned at creation
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: Uuid,
    pub make: String,
    pub model: String,
    pub plate_number: String,
    pub year: u16,
    pub driver_id: Uuid, // Foreign key to Driver
}

#[derive(Debug, Clone)]
pub struct DriverLocation {
    pub driver_id: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: chrono::NaiveDateTime,
}

// optional later: Add models for DriverScedule (if you want planned shifts for later), DriverEarnings, DriverPrerferences(max distance, max time, etc.)
