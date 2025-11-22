-- Database schema for URL Shortener

-- Drop tables if they exist (for clean reset)
DROP TABLE IF EXISTS subscriptions;
DROP TABLE IF EXISTS clicks;
DROP TABLE IF EXISTS urls;
DROP TABLE IF EXISTS users;

-- URLs table
CREATE TABLE urls (
  id TEXT PRIMARY KEY,
  short_code TEXT UNIQUE NOT NULL,
  original_url TEXT NOT NULL,
  user_id TEXT, -- Anonymous User ID from LocalStorage
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  clicks INTEGER DEFAULT 0
);

-- Clicks table for analytics
CREATE TABLE clicks (
  id TEXT PRIMARY KEY,
  url_id TEXT NOT NULL,
  clicked_at TEXT DEFAULT CURRENT_TIMESTAMP,
  ip_address TEXT,
  user_agent TEXT,
  referer TEXT,
  country TEXT,
  FOREIGN KEY (url_id) REFERENCES urls(id)
);

-- Indexes for performance
CREATE INDEX idx_urls_short_code ON urls(short_code);
CREATE INDEX idx_urls_user_id ON urls(user_id);
CREATE INDEX idx_clicks_url_id ON clicks(url_id);
CREATE INDEX idx_clicks_clicked_at ON clicks(clicked_at);
