// the brains: state mgmt + scoring + matching
// todo add more classes to this module, such as state, scoring, where more logic can go
// todo: for now we are using in memory caches, but we can swap out with redis or similar later

use common::events_schema::{DriverAssignedEvent, NoDriversAvailableEvent, RideRequestedEvent};
use common::redis_namespaces::DRIVER_LOCATION_NAMESPACE;
use common::subjects::{DRIVER_ASSIGNED_SUBJECT, NO_DRIVERS_AVAILABLE_SUBJECT};
use redis::geo::{RadiusOptions, RadiusOrder, RadiusSearchResult};
use redis::{geo, AsyncCommands};
use std::sync::Arc;
use tokio::time::Instant;

use crate::events::producers::EventProducer;

/// Core Matcher service
pub struct MatcherService {
    // The MultiplexedConnection is already designed to be shared safely across tasks and threads (it implements Clone, Send, and Sync).
    // but we wanted to wrap it in mutex for internal mutability when needed.
    redis_client: Arc<tokio::sync::Mutex<redis::aio::MultiplexedConnection>>,
    producer: Arc<EventProducer>, // used to publish MatchProposed etc.
}

impl MatcherService {
    pub fn new(
        producer: Arc<EventProducer>,
        redis_client: redis::aio::MultiplexedConnection,
    ) -> Self {
        Self {
            redis_client: Arc::new(tokio::sync::Mutex::new(redis_client)),
            producer,
        }
    }

    pub async fn handle_ride_requested(
        &self,
        event: RideRequestedEvent,
    ) -> Result<(), anyhow::Error> {
        // get all available drivers within the range
        // todo production level if the selected driver don't accept we have to try the next best driver etc. or if no drivers available
        // we have to increase searched radus etc.

        let mut redis_con = self.redis_client.lock().await;
        let opts = RadiusOptions::default().with_dist().order(RadiusOrder::Asc);
        let start = Instant::now();

        let redis_search_results: Vec<RadiusSearchResult> = redis_con
            .geo_radius(
                DRIVER_LOCATION_NAMESPACE,
                event.origin_lng,
                event.origin_lat,
                2.0,
                geo::Unit::Kilometers,
                opts,
            )
            .await?;
        let duration = start.elapsed();
        eprintln!("geo_radius took {:?}", duration);

        drop(redis_con); // release lock early

        let closest_driver = redis_search_results.first();
        // send event to that one driver (MatchProposedEvent)
        if let Some(driver) = closest_driver {
            eprintln!(
                "Closest driver to ride {} is driver {} at distance {:?} meters",
                event.ride_id, driver.name, driver.dist
            );

            let driver_assigned_event = DriverAssignedEvent {
                ride_id: event.ride_id,
                driver_id: uuid::Uuid::parse_str(&driver.name)?,
                pickup_lat: event.origin_lat,
                pickup_lng: event.origin_lng,
                assigned_at: event.created_at,
                dropoff_lat: event.destination_lat,
                dropoff_lng: event.destination_lng,
            };

            let payload = match serde_json::to_vec(&driver_assigned_event) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to serialize DriverAssignedEvent: {}", e);
                    // todo: publish no available drivers for this request event so that rider can be notified
                    return Err(anyhow::anyhow!(
                        "Failed to serialize DriverAssignedEvent: {}",
                        e
                    ));
                }
            };

            self.producer
                .publish(DRIVER_ASSIGNED_SUBJECT, &payload)
                .await?;
        } else {
            // todo publish no available drivers for this request event so that rider can be notified

            let no_driver_available_event = NoDriversAvailableEvent {
                ride_id: event.ride_id,
                rider_id: event.rider_id,
                requested_at: event.created_at,
                reason: Some("No available drivers in vicinity".to_string()),
            };
            let payload = match serde_json::to_vec(&no_driver_available_event) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to serialize NoDriversAvailableEvent: {}", e);
                    return Err(anyhow::anyhow!(
                        "Failed to serialize NoDriversAvailableEvent: {}",
                        e
                    ));
                }
            };

            self.producer
                .publish(NO_DRIVERS_AVAILABLE_SUBJECT, &payload)
                .await?;

            eprintln!(
                "No available drivers found for ride {} at location ({}, {})",
                event.ride_id, event.origin_lat, event.origin_lng
            );
        }

        Ok(())
    }
}
