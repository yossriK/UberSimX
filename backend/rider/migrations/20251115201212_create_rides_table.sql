CREATE TABLE IF NOT EXISTS rides (
    id                  UUID PRIMARY KEY,
    rider_id            UUID NOT NULL,
    
    origin_lat          DOUBLE PRECISION NOT NULL,
    origin_lng          DOUBLE PRECISION NOT NULL,
    destination_lat     DOUBLE PRECISION NOT NULL,
    destination_lng     DOUBLE PRECISION NOT NULL,

    status              TEXT NOT NULL,                   -- requested, matched, enroute, completed, cancelled
    driver_id           UUID,                            -- null until matched
    match_time          TIMESTAMPTZ,
    pickup_time         TIMESTAMPTZ,
    dropoff_time        TIMESTAMPTZ,

    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW()
);
