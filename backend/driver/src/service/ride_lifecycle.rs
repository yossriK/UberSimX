use std::sync::Arc;

use async_trait::async_trait;
use axum::Error;
use crate::events::schemas::DriverAssignedRideDto;
use crate::infra::repository::driver_status_repository::DriverStatusRepository;

#[async_trait]
pub trait RideLifeCycle: Send + Sync {
    async fn start_ride(&self) -> Result<(), Error>;
    async fn complete_ride(&self) -> Result<(), Error>;
    async fn handle_driver_assigned(
        &self,
        event: DriverAssignedRideDto,
    ) -> Result<(), Error>;
}
pub struct RideLifeCycleService {
    pub driver_status_repo: Arc<dyn DriverStatusRepository + Send + Sync>,
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

    async fn handle_driver_assigned(
        &self,
        event: DriverAssignedRideDto,
    ) -> Result<(), Error> {
        // Logic to handle driver assigned event
        println!("{:?}", event);

        // need to push notifications to the client/simulator to accept/reject ride

        // ride accepted then we notify driver and update status repo and redis

        // ride rejected we notify matcher to look for another driver and update status repo and redis
        // and notify rider as well
        Ok(())
    }
}
