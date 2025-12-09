// This file will hold redis key generation helper functions shared across the project.

use uuid::Uuid;

/// Returns the Redis namespace for driver state using the provided driver ID.
pub fn driver_state_namespace(driver_id: Uuid) -> String {
	format!("drivers:{}:state", driver_id)
}
