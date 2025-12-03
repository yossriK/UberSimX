CREATE TABLE driver_status (
    driver_id UUID PRIMARY KEY
        REFERENCES drivers(id) ON DELETE CASCADE,

    driver_available BOOLEAN NOT NULL DEFAULT FALSE,

    -- Ride lifecycle status
    ride_status TEXT NOT NULL CHECK (
        ride_status IN (
            'none',            -- not on a trip
            'assigned',        -- trip matched + assigned
            'pickup_arrived',
            'in_progress',     -- passenger onboard
            'completed'        -- just finished a trip
        )
    ),

    -- Associated trip, null if idle
    current_trip_id UUID NULL,
    
    status_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
