// gets called from consumer then delegates to matcher service
// gets called from matcher then produces to producer

use crate::{events::schema::RideRequested, matcher::service::MatcherService};

// previously this was a concrete struct with methods for each event type before traits
// pub struct EventHandler {
//     matcher: Arc<MatcherService>,
// }
// impl EventHandler {
//     pub fn new(matcher: Arc<MatcherService>) -> Self {
//         Self { matcher }
//     }
//     pub async fn on_ride_requested(&self, event: RideRequested) {
//         self.matcher.handle_ride_requested(event).await;
//     }
// }

#[async_trait::async_trait]
pub trait EventHandler<T> {
    async fn handle(&self, event: T);
}

#[async_trait::async_trait]
impl EventHandler<RideRequested> for MatcherService {
    async fn handle(&self, evt: RideRequested) {
        self.handle_ride_requested(evt).await;
    }
}
