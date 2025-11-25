// the brains: state mgmt + scoring + matching
// todo add more classes to this module, such as state, scoring, where more logic can go
// todo: for now we are using in memory caches, but we can swap out with redis or similar later

use redis::AsyncCommands;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::events::{producers::EventProducer, schema::RideRequestedEvent};
use crate::matcher::domain::{DriverState, RideRecord};

/// Core Matcher service
pub struct MatcherService {
    // The MultiplexedConnection is already designed to be shared safely across tasks and threads (it implements Clone, Send, and Sync).
    // but we wanted to wrap it in mutex for internal mutability when needed.
    redis_client: Arc<tokio::sync::Mutex<redis::aio::MultiplexedConnection>>,
    drivers: Arc<RwLock<HashMap<String, DriverState>>>,
    riders: Arc<RwLock<HashMap<String, RideRecord>>>,
    producer: Arc<EventProducer>, // used to publish MatchProposed etc.
}

impl MatcherService {
    pub fn new(
        producer: Arc<EventProducer>,
        redis_client: redis::aio::MultiplexedConnection,
    ) -> Self {
        Self {
            redis_client: Arc::new(tokio::sync::Mutex::new(redis_client)),
            drivers: Arc::new(RwLock::new(HashMap::new())),
            riders: Arc::new(RwLock::new(HashMap::new())),
            producer,
        }
    }

    pub async fn handle_ride_requested(
        &self,
        event: RideRequestedEvent,
    ) -> Result<(), anyhow::Error> {
        let ride_record = RideRecord::from(event);

        let key = format!("matcher:ride:{}", ride_record.ride_id);

        let value = serde_json::to_string(&ride_record)?;

        {
            let mut redis = self.redis_client.lock().await;
            redis.set::<_, _, ()>(&key, value).await?;
        }

        let stored: String = {
            let mut redis = self.redis_client.lock().await;
            redis.get::<_, String>(&key).await?
        };
        let ride_from_redis: RideRecord = serde_json::from_str(&stored)?;

        println!("Ride read from Redis: {:?}", ride_from_redis);

        Ok(())
    }
}
