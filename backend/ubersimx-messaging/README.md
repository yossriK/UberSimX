# ubersimx-messaging

This library provides a simple messaging abstraction for Rust services, initially wrapping [NATS](https://nats.io/) for event publishing and subscription. The design makes it easy to swap out the backend for other messaging systems like Kafka in the future, by abstracting messaging primitives behind a common API.

## Features

- Easy async publish/subscribe API
- Built-in NATS support via [nats.rs](https://github.com/nats-io/nats.rs)
- Ready to extend for other systems (e.g., Kafka)

## Usage

Add `ubersimx-messaging` as a dependency, then:

```rust
use ubersimx_messaging::MessagingClient;

#[tokio::main]
async fn main() {
    let client = MessagingClient::connect("localhost:4222").await.unwrap();
    client.publish("events.ride", b"hello world").await.unwrap();
    let mut subscription = client.subscribe("events.ride").await.unwrap();
    while let Some(msg) = subscription.next().await {
        println!("Received: {:?}", msg.data);
    }
}
```

## Future-proofing

The messaging API is designed so that you can swap the NATS backend for Kafka (or others) by only changing the implementation in `ubersimx-messaging`, not your service code.

## Running NATS Locally (with Docker)

1. Install [Docker](https://www.docker.com/).
2. Start a local NATS server:

   ```sh
   docker run -p 4222:4222 nats:latest
   ```

Your Rust services can now connect to `localhost:4222` for messaging.