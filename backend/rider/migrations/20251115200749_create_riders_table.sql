CREATE TABLE IF NOT EXISTS riders (
    rider_id        UUID PRIMARY KEY,
    name            TEXT NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);
