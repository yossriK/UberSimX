// This file will hold all event schemas shared across the project.
// Define your event structs and enums here.

use serde::Serialize;
use uuid::Uuid;


#[derive(Debug, Clone, Serialize)]
pub struct DriverAvailabilityChangedEvent {
    pub driver_id: Uuid,
    pub driver_available: bool,
}
