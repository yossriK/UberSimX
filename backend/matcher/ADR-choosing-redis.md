# **ADR: Choosing Redis GEO Instead of In-Memory R*-Tree + Journaling for Matcher State**

## **Status**

Accepted

## **Context**

During the design phase of the Matcher Service for UberSimX, we evaluated using an **in-memory R*-tree** to maintain spatial data for active drivers and pending rides. This would have delivered extremely fast spatial lookups, but it introduced major reliability and scalability issues.

Because an in-memory R*-tree is **per-instance state**, each matcher instance would maintain its own copy of the driver and ride data. This immediately raised two severe problems:

### **1. Horizontal Scaling Becomes Nearly Impossible**

If you run two or more matcher instances:

* Each instance would have its **own** R*-tree.
* Each instance would receive events at slightly different times.
* Their internal state would inevitably diverge.
* Synchronizing trees between instances is effectively impossible without a shared external store.

This means the architecture **could not scale** without introducing extreme complexity.

### **2. Consistency Between Matcher Instances Cannot Be Maintained**

Even with journaling, snapshots, and JetStream replay:

* Matchers could still be out of sync if they received events in different orders.
* Recovery timing differences could cause mismatched driver positions.
* Tiny time offsets would accumulate into major inconsistencies.

Ensuring that multiple matchers hold *identical* in-memory R*-trees is not realistic in a distributed system.

---

To mitigate data loss within a single instance, we considered a persistence strategy:

### **A. Journaling (Proposed)**

Append every event to PostgreSQL.

### **B. Snapshots (Proposed)**

Periodically persist the entire R*-tree to reduce replay time.

### **C. JetStream Durable Consumers**

Replay missed events after a crash using consumer offsets.

Recovery would have required:

1. Loading the latest snapshot
2. Replaying the journal after the snapshot
3. Replaying unacknowledged JetStream events
4. Rebuilding the R*-tree in memory

This would protect a **single** matcher instance from data loss — but it did **nothing** to solve horizontal scaling or cross-instance consistency problems.

---

## **Decision**

We replaced the entire R*-tree + journal + snapshot design with **Redis GEO** as the authoritative spatial database.

### **Why Redis GEO?**

* Provides a **single, shared, durable** source of truth
* Eliminates cross-instance consistency issues
* Makes horizontal scaling trivial: all matcher instances read/write to the same dataset
* No need for journals or snapshots
* Crash recovery is automatic — there is nothing in memory to rebuild
* Redis GEO commands offer fast, production-proven spatial querying

JetStream remains only for event delivery — ensuring that if the matcher is down, missed events will be replayed and applied to Redis.

---

## **Consequences**

### **Pros**

* Architecture becomes dramatically simpler
* Horizontal scaling becomes fully supported
* No risk of inconsistent state across matcher instances
* No custom persistence logic required
* Startup and recovery times are instant
* Operational overhead is greatly reduced

### **Cons**

* Redis is now a critical external dependency
* Requires HA Redis deployment for production

---

## **Alternative Considered: Geographic Sharding**

We also evaluated **geographic sharding** as a scaling option.

### **Concept**

* Divide the map into geographic tiles
* Run one matcher per tile
* Each matcher stores only the data for its assigned region

### **Why It Was Rejected**

* Riders near the borders need multi-shard queries
* Drivers constantly move across boundaries
* Requires complex inter-shard communication
* Adds operational overhead with questionable benefits for a simulation

Sharding is suitable for massive real-world fleets, but unnecessary and overly complex for UberSimX.

---

## **Summary**

The in-memory R*-tree approach would have required journals, snapshots, and event replay just to avoid losing data in a **single instance**, while failing to address the far bigger problem: **consistency and scalability across multiple matcher instances.**

Redis GEO solves these problems by providing:

* A durable, centralized, consistent spatial store
* Easy horizontal scaling
* Simple crash recovery
* Eliminated need for custom persistence mechanisms

This makes Redis GEO the most reliable, maintainable, and scalable solution for the Matcher Service.

---