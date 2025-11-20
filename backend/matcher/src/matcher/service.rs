// the brains: state mgmt + scoring + matching
// todo add more classes to this module, such as state, scoring, where more logic can go
// todo: for now we are using in memory caches, but we can swap out with redis or similar later

use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::events::{producers::EventProducer, schema::RideRequested};

// Basic driver state
#[derive(Debug, Clone)]
struct DriverState {
    driver_id: String,
    lat: f64,
    lon: f64,
    available: bool,
}

#[derive(Debug, Clone)]
struct RiderState {
    ride_id: Uuid,
    rider_id: Uuid,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub created_at: DateTime<Utc>,
    pub status: RideStatus,
}

#[derive(Debug, Clone)]
enum RideStatus {
    Pending,
    Matched,
    Expired
}

/// Core Matcher service
pub struct MatcherService {
    drivers: Arc<RwLock<HashMap<String, DriverState>>>,
    riders: Arc<RwLock<HashMap<String, RiderState>>>,
    producer: Arc<EventProducer>, // used to publish MatchProposed etc.
}

impl MatcherService {
    pub fn new(producer: Arc<EventProducer>) -> Self {
        Self {
            drivers: Arc::new(RwLock::new(HashMap::new())),
            riders: Arc::new(RwLock::new(HashMap::new())),
            producer,
            // todo optional: istead of callign producer directly, could send to a mpsc channel and have separate task do publishing, like
            // evnet dispatcher, which then calls producer. Ill worry about it later
        }
    }

    pub async fn handle_ride_requested(&self, event: RideRequested) {
        // store rider state. After Rider has been matched with driver, we can either remove from db or update status to matched. 
        // the latter might be useful for analytics later, pretain history of rides etc (soft delete)
        let rider_state = RiderState {
            ride_id: event.ride_id.clone(),
            rider_id: event.rider_id.clone(),
            origin_lat: event.origin_lat,
            origin_lng: event.origin_lng,
            destination_lat: event.destination_lat,
            destination_lng: event.destination_lng,
            created_at: event.created_at,
            status: RideStatus::Pending,
        };

        // add db connection here and store in db

        // todo: so what do I do here? find nearby drivers? emit another event? score them, propose match

        println!("Received ride request in the matcher wow: {:?}", event);
    }
}
