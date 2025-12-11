// Event subjects shared across the backend
pub const RIDE_REQUESTED_SUBJECT: &str = "rider.ride.requested";
pub const DRIVER_AVAILABILITY_SUBJECT: &str = "driver.availability.changed";
pub const DRIVER_ASSIGNED_SUBJECT: &str = "driver.ride.assigned";
pub const NO_DRIVERS_AVAILABLE_SUBJECT: &str = "rider.ride.no_drivers_available";
pub const DRIVER_ACCEPTED_RIDE_SUBJECT: &str = "driver.ride.accepted";
pub const DRIVER_REJECTED_RIDE_SUBJECT: &str = "driver.ride.rejected";