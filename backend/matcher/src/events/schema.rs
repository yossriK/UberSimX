use serde::Deserialize;
use serde::Serialize;
// defines all event types (RideRequested, DriverLocationUpdated, etc.)

#[derive(Debug, Serialize, Deserialize)]
pub struct RideRequested {
    pub ride_id: String,
    pub rider_id: String,
    pub origin: (f64, f64),
    pub dest: (f64, f64),
    pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchProposed {
    pub ride_id: String,
    pub driver_id: String,
    pub rider_id: String,
}
