-- Migration script to initialize the database schema

-- Drivers table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE drivers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    license_number TEXT,
    rating REAL, -- f32 maps to REAL in Postgres
    car_id UUID,
    created_at TIMESTAMP DEFAULT NOW()
);