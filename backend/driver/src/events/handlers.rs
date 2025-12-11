// gets called from consumer then delegates to matcher service
// gets called from matcher then produces to producer

use crate::events::schemas::DriverAssignedRideDto;
use crate::service::ride_lifecycle::RideLifeCycle;
use crate::service::ride_lifecycle::RideLifeCycleService;
use common::events_schema::DriverAssignedRideEvent;

#[async_trait::async_trait]
pub trait EventHandler<T> {
    async fn handle(&self, event: T);
}

#[async_trait::async_trait]
impl EventHandler<DriverAssignedRideEvent> for RideLifeCycleService {
    async fn handle(&self, evt: DriverAssignedRideEvent) {
        // Mapping from event to domain model is the responsibility of the service method handle_ride_requested, not the generic handler.
        // Convert event to DTO before passing to the service method
        let dto = DriverAssignedRideDto::from(&evt);
        if let Err(e) = self.handle_driver_assigned(dto).await {
            eprintln!("Error handling DriverAssignedEvent: {:?}", e);
        }
    }
}
