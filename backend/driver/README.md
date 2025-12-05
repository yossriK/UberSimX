# To view driver locations in Redis:
```
redis-cli GEOPOS drivers:locations <driver_id>
```
Replace `<driver_id>` with the actual driver ID.



we have a simulation controller, not a real app calling the driver service, this will affect our design a little bit. 
For example, there will be no need for a /accept endpoint where driver chooses to accept or reject a ride after
a mathc been made, we will just assume that all rides are accepted after matcher service finds a match 
between closest driver and rider. 
We also are not too strict on driver filters in terms of car type, luxury, pets allowed, rating etc... 
those details can come in later.

also flags like is driver banned, blocked, cancelled too many rides and now on cooldown status, were not considered initially. 

FULL EVENT FLOW (SIMPLE DIAGRAM)
Driver App  
   ↓  
Driver Service  
   ↓ location update  
Redis GEOSET  
   ↑ (read by matcher)
Matcher Service  
   ↓ publish RideAssignedEvent  
Driver Service (consumes)
   ↓ driver picks up  
DriverPickedUpEvent  
   ↓  
Ride Service  
   ↓ driver completes  
DriverRideCompletedEvent  
   ↓  
Ride Service marks ride done  
   ↓  
Matcher consumes event ⇒ makes driver available again



In Driver service Postgres is the source of truth for anything that is business logic, auditable, or long-lived.
So driver status absolutely belongs here.  so we store availability_status, ride_status, current_trip_id, status_updated_at. However we don't store Last known location, last seen (heartbeat update timestamp when locaiton/status changes)


If we are using grpc we can use client streaming so the drivers phone is always streaming location instead of calling the endpoint