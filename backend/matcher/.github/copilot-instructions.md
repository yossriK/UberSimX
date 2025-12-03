# Copilot Instructions for UberSimX Matcher Service

## Project Overview
- This is the Matcher Service for UberSimX, responsible for matching ride requests to drivers.
- The service is event-driven, using NATS JetStream for event delivery and Redis GEO for spatial state management.
- The codebase is organized by domain: `events/` for event schemas and consumers, `matcher/` for business logic and domain models, and `api/` for (optional) admin/debug endpoints.

## Architecture & Data Flow
- **Events**: Incoming events (e.g., `RideRequestedEvent`) are received via NATS JetStream subscriptions (see `events/consumers.rs`).
- **Domain Models**: Event structs (in `events/schema.rs`) are mapped to domain structs (in `matcher/domain.rs`, e.g., `RideRecord`).
- **State**: State is managed in Redis using namespaced keys (e.g., `matcher:ride:{ride_id}`) and in-memory caches for drivers/riders.
- **Outgoing Events**: Domain events are published back to NATS via the `EventProducer` (see `events/producers.rs`).

## Key Patterns & Conventions
- **Event Handling**: Use the `EventHandler<T>` trait (in `events/handler.rs`) for dispatching events to the service layer. Error handling is done in the trait impl.
- **Domain Mapping**: Implement `From<Event> for DomainModel` in `matcher/domain.rs` for clean conversion between event and domain types.
- **Redis Usage**: Use a single `MultiplexedConnection` (async, shared via `Arc<Mutex<...>>`). Pooling is not used unless benchmarking shows a bottleneck.
- **Key Namespacing**: All Redis keys are prefixed (e.g., `matcher:ride:`) to avoid collisions with other services.
- **Configuration**: Environment variables are loaded from `settings.env` (see `main.rs`).
- **External Dependencies**: NATS JetStream (for messaging), Redis (for state), see ADRs for rationale.

## Developer Workflows
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: (No tests yet; add to `tests/` or as integration tests.)
- **Lint/Fix Imports**: `cargo fix --allow-dirty --allow-staged`
- **Environment**: Copy or edit `settings.env` for local config.

## Important Files
- `src/main.rs`: Service entrypoint, sets up dependencies and event consumers.
- `src/events/schema.rs`: Event/message structs for inter-service communication.
- `src/matcher/domain.rs`: Domain models and conversion logic.
- `src/matcher/service.rs`: Core business logic and state management.
- `src/events/consumers.rs`: NATS event subscriptions and dispatch.
- `ADR-choosing-redis.md`, `ADR-choosing-NATS-JetStream.md`: Architectural decisions and rationale.

## Integration Points
- **Messaging**: Uses `ubersimx-messaging` crate for NATS JetStream integration. Event subjects are defined as constants in that crate.
- **Redis**: All stateful operations use async Redis commands. Data is serialized as JSON.

## Project-Specific Notes
- Do not use in-memory state for production; Redis is the source of truth.
- All event and domain structs derive `Serialize`/`Deserialize` for easy (de)serialization.
- Service is designed for horizontal scaling; see ADRs for details.

---
For more details, see the ADRs and comments in `main.rs`, `domain.rs`, and `service.rs`.
