// This file will hold redis namespace constants shared across the project.


// NAMESPACE CONSTANTS
pub const DRIVER_LOCATION_NAMESPACE: &str = "drivers:locations";

// KEYS CONSTANTS
pub const DRIVER_LAST_LOCATION_UPDATE_FIELD: &str = "last_location_ts";
pub const DRIVER_LAST_AVAILABILITY_UPDATE_FIELD: &str = "last_updated";
pub const DRIVER_AVAILABILITY_FIELD: &str = "available";
pub const DRIVER_AVAILABILITY_REASON_FIELD: &str = "reason";
pub const DRIVER_IN_RIDE_FIELD: &str = "in_ride";
pub const DRIVER_RIDE_ID_FIELD: &str = "ride_id";