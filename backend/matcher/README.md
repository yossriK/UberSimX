## Installing and Running NATS JetStream with Docker

To install the NATS Docker image, run:

```bash
docker pull nats:latest
```

To start a NATS server with JetStream enabled:

```bash
docker run -d --name nats-server -p 4222:4222 nats:latest -js
```

This will start NATS with JetStream enabled and expose it on port 4222.


## Matcher Service Driver Data Policy

- **Driver Location Ownership:**
	- The driver service is the sole owner of driver location data in Redis. It is the only service permitted to write or update driver location. This strict ownership ensures that location data remains consistent, accurate, and traceable to the source of truth: the driver service itself.

- **Matcher Service Read-Only Access:**
	- The matcher service must only read driver location data from Redis. Under no circumstances should it write, modify, or overwrite driver location. Allowing multiple services to write location data would lead to race conditions, conflicting updates, and ultimately inconsistent or unreliable location information across the system.

- **Availability as Computed Data:**
	- The matcher service is permitted to write driver availability status to Redis. Availability is a computed property, derived from events and business logic (such as ride assignments, driver status changes, etc.). Since availability is not a direct input from the driver, but rather a result of system state and event processing, the matcher service can safely update this field without risking data inconsistency.

- **Service Boundaries and Data Integrity:**
	- By enforcing these boundaries—driver service owns location, matcher service computes and writes availability—the system maintains clear responsibilities and avoids the pitfalls of shared mutable state. This approach is critical for maintaining data integrity, especially in distributed or horizontally scaled deployments.
