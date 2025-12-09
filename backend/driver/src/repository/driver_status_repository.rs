use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{DriverStatus, RideStatus};



#[async_trait]
pub trait DriverStatusRepository {
    async fn create_status(&self, status: &DriverStatus) -> Result<(), Error>;
    async fn delete_status(&self, driver_id: Uuid) -> Result<(), Error>;
    async fn patch_status(
        &self,
        driver_id: Uuid,
        driver_available: Option<bool>,
        ride_status: Option<RideStatus>,
        current_trip_id: Option<Option<Uuid>>,
    ) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct PgDriverStatusRepository {
    pub pool: Arc<PgPool>,
}

impl PgDriverStatusRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DriverStatusRepository for PgDriverStatusRepository {
    async fn create_status(&self, status: &DriverStatus) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO driver_status (driver_id, driver_available, ride_status, current_trip_id)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(status.driver_id)
        .bind(status.driver_available)
        .bind(status.ride_status.to_string())
        .bind(status.current_trip_id)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    async fn delete_status(&self, driver_id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM driver_status WHERE driver_id = $1")
            .bind(driver_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    async fn patch_status(
        &self,
        driver_id: Uuid,
        driver_available: Option<bool>,
        ride_status: Option<RideStatus>,
        current_trip_id: Option<Option<Uuid>>,
    ) -> Result<(), Error> {
        let mut sets = Vec::new();
        let mut param_index = 1;

        // Store the values to bind in order
        let mut bind_driver_available = None;
        let mut bind_ride_status = None;
        let mut bind_current_trip_id = None;

        if let Some(val) = driver_available {
            sets.push(format!("driver_available = ${}", param_index));
            param_index += 1;
            bind_driver_available = Some(val);
        }
        if let Some(ref val) = ride_status {
            sets.push(format!("ride_status = ${}", param_index));
            param_index += 1;
            bind_ride_status = Some(val.clone());
        }
        if let Some(val) = current_trip_id {
            sets.push(format!("current_trip_id = ${}", param_index));
            param_index += 1;
            bind_current_trip_id = Some(val);
        }

        // Always update status_updated_at
        sets.push("status_updated_at = NOW()".to_string());

        if sets.len() == 1 {
            // Only status_updated_at would be updated, so nothing to patch
            return Ok(());
        }

        let query = format!(
            "UPDATE driver_status SET {} WHERE driver_id = ${}",
            sets.join(", "),
            param_index
        );
        let mut q = sqlx::query(&query);

        if let Some(val) = bind_driver_available {
            q = q.bind(val);
        }
        if let Some(val) = bind_ride_status {
            q = q.bind(val.to_string());
        }
        if let Some(val) = bind_current_trip_id {
            q = q.bind(val);
        }
        q = q.bind(driver_id);

        q.execute(self.pool.as_ref()).await?;
        Ok(())
    }
}
