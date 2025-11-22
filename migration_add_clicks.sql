-- Migration: Recreate clicks table with enhanced analytics

-- Drop old clicks table
DROP TABLE IF EXISTS clicks;

-- Create new clicks table with detailed analytics fields
CREATE TABLE clicks (
    id TEXT PRIMARY KEY,
    short_code TEXT NOT NULL,
    clicked_at TEXT DEFAULT CURRENT_TIMESTAMP,
    country TEXT,
    city TEXT,
    device_type TEXT,
    browser TEXT,
    os TEXT,
    referrer TEXT,
    ip_hash TEXT
);

-- Create indexes for performance
CREATE INDEX idx_clicks_short_code ON clicks(short_code);
CREATE INDEX idx_clicks_timestamp ON clicks(clicked_at);
CREATE INDEX idx_clicks_country ON clicks(country);
