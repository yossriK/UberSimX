use std::sync::Arc;

use crate::events::publisher::EventPublisher;
use crate::events::schemas::DriverAssignedRideDto;
use crate::infra::repository::driver_status_repository::DriverStatusRepository;
use crate::models::AvailabilityReason;
use crate::models::RideStatus;
use anyhow::anyhow;
use anyhow::Error;
use async_trait::async_trait;
use common::redis_key_helpers::driver_state_namespace;
use common::redis_namespaces::{
    DRIVER_AVAILABILITY_FIELD, DRIVER_AVAILABILITY_REASON_FIELD,
    DRIVER_LAST_AVAILABILITY_UPDATE_FIELD,
};

#[async_trait]
pub trait RideLifeCycle: Send + Sync {
    async fn start_ride(&self) -> Result<(), Error>;
    async fn complete_ride(&self) -> Result<(), Error>;
    async fn handle_driver_assigned(&self, event: DriverAssignedRideDto) -> Result<(), Error>;
}
pub struct RideLifeCycleService {
    pub driver_status_repo: Arc<dyn DriverStatusRepository + Send + Sync>,
    pub(crate) producer: Arc<EventPublisher>,
    pub redis_con: Arc<tokio::sync::Mutex<redis::aio::MultiplexedConnection>>,
}

#[async_trait]
impl RideLifeCycle for RideLifeCycleService {
    async fn start_ride(&self) -> Result<(), Error> {
        // Logic to start a ride
        Ok(())
    }

    async fn complete_ride(&self) -> Result<(), Error> {
        // Logic to complete a ride
        Ok(())
    }

    async fn handle_driver_assigned(&self, event: DriverAssignedRideDto) -> Result<(), Error> {
        // Logic to handle driver assigned event
        println!("{:?}", event);

        // update driver status to assigned in the driver status repository and redis so matcher don't match this driver for other rides
        
        self.driver_status_repo
            .patch_status(
                event.driver_id,
                Some(false),
                Some(RideStatus::Assigned),
                Some(event.ride_id),
            )
            .await?;

        let key = driver_state_namespace(event.driver_id);

        let mut con = self.redis_con.lock().await;
        let mut pipe = redis::pipe();
        pipe.atomic()
            .hset(&key, DRIVER_AVAILABILITY_FIELD, false)
            .hset(
                &key,
                DRIVER_AVAILABILITY_REASON_FIELD,
                AvailabilityReason::RideAssigned.to_string(),
            )
            .hset(
                &key,
                DRIVER_LAST_AVAILABILITY_UPDATE_FIELD,
                chrono::Utc::now().timestamp(),
            )
            .expire(&key, 90);
        pipe.query_async::<()>(&mut *con)
            .await
            .map_err(|_| anyhow!("Failed to update driver status in Redis"))?;


        
        // need to push notifications to the client/simulator to accept/reject ride

        // ride accepted then we notify driver and update status repo and redis

        // ride rejected we notify matcher to look for another driver and update status repo and redis
        // and notify rider as well
        Ok(())
    }
}
