# **ADR: Using NATS JetStream for Event Delivery in Matcher Service**

## **Status**

Accepted

## **Context**

The Matcher Service requires reliable delivery of events from:

* **Rider Service** (`ride.requested`, `ride.cancelled`)
* **Driver Service** (`driver.available`, `driver.location_updated`, `driver.offline`)

Key requirements for event delivery:

1. **Durability:** Matcher must not lose events even if it crashes.
2. **Replayability:** Matcher must be able to process events it missed while offline.
3. **Ordering:** Events should be processed in the correct order per stream (especially driver location updates).
4. **Scalability:** Multiple matcher instances should be able to consume events without interference.

---

## **Decision**

We decided to use **NATS JetStream** for event delivery between services.

### **Why JetStream?**

* **Durable Consumers:** JetStream tracks the last acknowledged message per consumer, allowing matcher instances to resume exactly where they left off.
* **Message Replay:** If the matcher crashes or is restarted, JetStream delivers all unacknowledged events, ensuring no data loss.
* **Exactly-once or At-least-once Semantics:** Combined with idempotent event handling, JetStream guarantees safe processing.
* **Lightweight & Fast:** NATS has low latency and high throughput, ideal for frequent driver location updates.
* **Multi-instance Support:** Multiple matchers can independently consume the same stream without conflicting with each other.

---

## **Operational Notes**

* Each matcher instance uses a **durable consumer** for each event type (`drivers`, `rides`).
* Matcher only acknowledges an event after successfully updating Redis (or performing business logic).
* In the rare case of partial failure (matcher crashes mid-processing), JetStream automatically re-delivers unacknowledged events.
* JetStream offsets eliminate the need for a custom journal for event recovery.

---

## **Consequences**

### **Pros**

* Reliable, replayable event delivery without building custom persistence
* Ensures no missed events during downtime
* Supports horizontal scaling with multiple matcher instances
* Low-latency, production-ready messaging
* Simplifies crash recovery logic

### **Cons**

* JetStream is an external dependency and must be deployed in a HA setup for production
* Matcher logic must be idempotent to safely handle potential redeliveries

---

## **Summary**

Using **NATS JetStream** provides a robust, production-ready solution for event delivery. Combined with Redis GEO as the authoritative state store, this design ensures:

* No events are lost even during crashes
* Multi-instance matchers stay consistent
* Event replay is automatic
* No need for custom journals or snapshots

This approach balances simplicity, reliability, and scalability for the Matcher Service.