// the brains: state mgmt + scoring + matching
// todo add more classes to this module, such as state, scoring, where more logic can go
// todo: for now we are using in memory caches, but we can swap out with redis or similar later

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::events::{producers::EventProducer, schema::RideRequestedEvent};
use crate::matcher::domain::{DriverState, RideRecord, RideStatus};

/// Core Matcher service
pub struct MatcherService {
    drivers: Arc<RwLock<HashMap<String, DriverState>>>,
    riders: Arc<RwLock<HashMap<String, RideRecord>>>,
    producer: Arc<EventProducer>, // used to publish MatchProposed etc.
}

impl MatcherService {
    pub fn new(producer: Arc<EventProducer>) -> Self {
        Self {
            drivers: Arc::new(RwLock::new(HashMap::new())),
            riders: Arc::new(RwLock::new(HashMap::new())),
            // todo optional: istead of callign producer directly, could send to a mpsc channel and have separate task do publishing, like
            // evnet dispatcher, which then calls producer. Ill worry about it later
            producer,
        }
    }

    pub async fn handle_ride_requested(&self, event: RideRequestedEvent) {
        // store rider state. After Rider has been matched with driver, we can either remove from db or update status to matched.
        // the latter might be useful for analytics later, pretain history of rides etc (soft delete)
        let ride_record = RideRecord::from(event);

        // add redis connection here and store in redis

        // todo: so what do I do here? find nearby drivers? emit another event? score them, propose match

        println!("Received ride request in the matcher wow: {:?}", ride_record);
    }
}
