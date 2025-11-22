-- URL Shortener Database Schema

-- URLs table
CREATE TABLE IF NOT EXISTS urls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    short_code TEXT UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    clicks INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_urls_short_code ON urls(short_code);
CREATE INDEX IF NOT EXISTS idx_urls_user_id ON urls(user_id);

-- Detailed clicks table for analytics
CREATE TABLE IF NOT EXISTS clicks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    short_code TEXT NOT NULL,
    clicked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    country TEXT,
    city TEXT,
    device_type TEXT,
    browser TEXT,
    os TEXT,
    referrer TEXT,
    ip_hash TEXT,
    FOREIGN KEY (short_code) REFERENCES urls(short_code) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_clicks_short_code ON clicks(short_code);
CREATE INDEX IF NOT EXISTS idx_clicks_timestamp ON clicks(clicked_at);
CREATE INDEX IF NOT EXISTS idx_clicks_country ON clicks(country);
