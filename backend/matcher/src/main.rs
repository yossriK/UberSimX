mod api;
mod events;
mod matcher;

use std::sync::Arc;

use ubersimx_messaging::messagingclient::MessagingClient;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Connect to the messaging service
    // todo properly configure the URL via env var or config file and handle the error
    let messaging_client = Arc::new(MessagingClient::connect("localhost:4222").await.unwrap());

    // setup the producer (for outgoing events)
    let producer = Arc::new(events::producers::EventProducer::new(
        messaging_client.clone(),
    ));

    // setup the matcher service (business logic)
    let matcher_service = Arc::new(matcher::service::MatcherService::new(producer.clone()));

    // setup the consumers (incoming events)
    let consumers = events::consumers::Consumers::new(messaging_client.clone());
    consumers.register_all(matcher_service.clone()).await;
}
