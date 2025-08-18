use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Driver {
    pub id: Uuid,
    pub name: String,
    pub license_number: String,
    pub rating: f32,
    pub(crate) car_id: Option<Uuid>,
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

#[derive(Debug, Clone)]
pub enum DriverStatus {
    Available,
    Unavailable,
    OnDuty,
    OffDuty,
}

#[derive(Debug, Clone)]
pub struct DriverAvailabilityEvent {
    pub driver_id: i32,
    pub status_change: DriverStatus,
    pub timestamp: chrono::NaiveDateTime,
}


// optional later: Add models for DriverScedule (if you want planned shifts for later), DriverEarnings, DriverPrerferences(max distance, max time, etc.)