use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use serde_json::json;
use crate::auth::get_stored_token;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortenRequest {
    pub url: String,
    pub custom_alias: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortenResponse {
    pub short_url: String,
    pub short_code: String,
    pub original_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlInfo {
    pub id: String,
    pub short_code: String,
    pub original_url: String,
    pub created_at: String,
    pub clicks: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnalyticsData {
    pub total_clicks: i64,
    pub countries: Vec<serde_json::Value>,
    pub devices: Vec<serde_json::Value>,
    pub browsers: Vec<serde_json::Value>,
    pub timeline: Vec<serde_json::Value>,
    pub referrers: Vec<serde_json::Value>,
}

// Helper to get API URL (assumes same origin or configured)
// For local dev with Trunk proxy, we can use relative paths.
// For production, it might be different, but let's assume relative for now or env var.
const API_BASE: &str = "https://s.blatik-short.workers.dev"; // Hardcoded for now based on existing config

pub async fn shorten_url(url: String, custom_alias: Option<String>, user_id: Option<String>) -> Result<ShortenResponse, String> {
    let body = ShortenRequest { url, custom_alias };
    
    let mut req = Request::post(&format!("{}/api/shorten", API_BASE));
        
    if let Some(uid) = user_id {
        req = req.header("X-User-ID", &uid);
    }
    
    if let Some(token) = get_stored_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    
    let request = req.json(&body).map_err(|e| e.to_string())?;
    let resp = request.send().await.map_err(|e| e.to_string())?;
    
    if !resp.ok() {
        let err_json: serde_json::Value = resp.json().await.unwrap_or(json!({"error": "Unknown error"}));
        return Err(err_json["error"].as_str().unwrap_or("Unknown error").to_string());
    }
    
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_user_urls(user_id: String) -> Result<Vec<UrlInfo>, String> {
    let mut req = Request::get(&format!("{}/api/urls", API_BASE))
        .header("X-User-ID", &user_id);
        
    if let Some(token) = get_stored_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
        
    let resp = req.send().await.map_err(|e| e.to_string())?;
    
    if !resp.ok() {
        return Err("Failed to fetch URLs".to_string());
    }
    
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_analytics(code: String) -> Result<AnalyticsData, String> {
    let resp = Request::get(&format!("{}/api/analytics/{}", API_BASE, code))
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !resp.ok() {
        return Err("Failed to fetch analytics".to_string());
    }
    
    resp.json().await.map_err(|e| e.to_string())
}
