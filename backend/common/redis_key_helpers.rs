// This file will hold redis key generation helper functions shared across the project.

use uuid::Uuid;

/// Returns the Redis namespace for driver metadata using the provided driver ID.
pub fn driver_metadata_namespace(driver_id: Uuid) -> String {
	format!("drivers:{}:metadata", driver_id)
}
