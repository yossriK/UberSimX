use std::sync::Arc;

use crate::events::publisher::EventPublisher;
use crate::events::schemas::DriverAssignedRideDto;
use crate::infra::repository::driver_status_repository::DriverStatusRepository;
use crate::models::AvailabilityReason;
use crate::models::RideStatus;
use crate::service::eta_service::EtaCalculator;
use crate::service::eta_service::EtaService;
use anyhow::anyhow;
use anyhow::Error;
use async_trait::async_trait;
use common::events_schema::DriverAcceptedRideEvent;
use common::events_schema::DriverRejectedRideEvent;
use common::redis_key_helpers::driver_state_namespace;
use common::redis_namespaces::DRIVER_IN_RIDE_FIELD;
use common::redis_namespaces::DRIVER_RIDE_ID_FIELD;
use common::redis_namespaces::{
    DRIVER_AVAILABILITY_FIELD, DRIVER_AVAILABILITY_REASON_FIELD,
    DRIVER_LAST_AVAILABILITY_UPDATE_FIELD,
};
use common::subjects::DRIVER_ACCEPTED_RIDE_SUBJECT;
use common::subjects::DRIVER_REJECTED_RIDE_SUBJECT;
use uuid::Uuid;

#[async_trait]
pub trait RideLifeCycle: Send + Sync {
    async fn start_ride(&self) -> Result<(), Error>;
    async fn complete_ride(&self) -> Result<(), Error>;
    async fn handle_driver_assigned(&self, event: DriverAssignedRideDto) -> Result<(), Error>;
    async fn handle_driver_accept_ride_assignment(
        &self,
        driver_id: Uuid,
        ride_id: Uuid,
    ) -> Result<(), Error>;

    async fn handle_driver_reject_ride_assignment(
        &self,
        driver_id: Uuid,
        ride_id: Uuid,
    ) -> Result<(), Error>;
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

        // reactions to driver accept/reject will be handled in separate methods
        Ok(())
    }

    async fn handle_driver_accept_ride_assignment(
        &self,
        driver_id: Uuid,
        ride_id: Uuid,
    ) -> Result<(), Error> {

        // - Update driver status repo and redis
        self.driver_status_repo
            .patch_status(
                driver_id,
                Some(false),
                Some(RideStatus::InRide),
                Some(ride_id),
            )
            .await?;

        let key = driver_state_namespace(driver_id);

        let mut con = self.redis_con.lock().await;
        let mut pipe = redis::pipe();
        pipe.atomic()
            .hset(&key, DRIVER_AVAILABILITY_FIELD, false)
            .hset(
                &key,
                DRIVER_AVAILABILITY_REASON_FIELD,
                AvailabilityReason::InRide.to_string(),
            )
            .hset(&key, DRIVER_IN_RIDE_FIELD, true)
            .hset(&key, DRIVER_RIDE_ID_FIELD, ride_id.to_string())
            .hset(
                &key,
                DRIVER_LAST_AVAILABILITY_UPDATE_FIELD,
                chrono::Utc::now().timestamp(),
            )
            .expire(&key, 90);
        pipe.query_async::<()>(&mut *con)
            .await
            .map_err(|_| anyhow!("Failed to update driver status in Redis"))?;



        // - Notify matcher to look for another driver
        let accepted_event = DriverAcceptedRideEvent {
            driver_id,
            ride_id,
            accepted_at: chrono::Utc::now(),
            estimated_pickup_time_minutes: EtaService{}.calculate_eta_minutes(),
        };

        let payload = serde_json::to_vec(&accepted_event)?;

        // - Notify rider service and matcher service about the acception
        self.producer
            .publish(DRIVER_ACCEPTED_RIDE_SUBJECT, payload)
            .await?;

        Ok(())
    }

    async fn handle_driver_reject_ride_assignment(
        &self,
        driver_id: Uuid,
        ride_id: Uuid,
    ) -> Result<(), Error> {
        // - Update driver status repo and redis
        self.driver_status_repo
            .patch_status(driver_id, Some(true), Some(RideStatus::None), None)
            .await?;

        let key = driver_state_namespace(driver_id);

        let mut con = self.redis_con.lock().await;
        let mut pipe = redis::pipe();
        pipe.atomic()
            .hset(&key, DRIVER_AVAILABILITY_FIELD, true)
            .hset(
                &key,
                DRIVER_AVAILABILITY_REASON_FIELD,
                AvailabilityReason::Available.to_string(),
            )
            .hset(&key, DRIVER_IN_RIDE_FIELD, false)
            .hset(&key, DRIVER_RIDE_ID_FIELD, "".to_string())
            .hset(
                &key,
                DRIVER_LAST_AVAILABILITY_UPDATE_FIELD,
                chrono::Utc::now().timestamp(),
            )
            .expire(&key, 90);
        pipe.query_async::<()>(&mut *con)
            .await
            .map_err(|_| anyhow!("Failed to update driver status in Redis"))?;

        // - Notify matcher to look for another driver

        let reject_event = DriverRejectedRideEvent { driver_id, ride_id };

        let payload = serde_json::to_vec(&reject_event)?;

        // - Notify rider service and matcher service about the rejection
        self.producer
            .publish(DRIVER_REJECTED_RIDE_SUBJECT, payload)
            .await?;

        Ok(())
    }
}
