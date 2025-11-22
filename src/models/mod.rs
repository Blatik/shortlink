use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub api_key: String,
    pub subscription_tier: SubscriptionTier,
    pub created_at: String,
    pub stripe_customer_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionTier {
    Free,
    Pro,
    Business,
}

impl SubscriptionTier {
    pub fn urls_per_month(&self) -> i32 {
        match self {
            SubscriptionTier::Free => 100,
            SubscriptionTier::Pro => 10_000,
            SubscriptionTier::Business => -1, // unlimited
        }
    }

    pub fn analytics_retention_days(&self) -> i32 {
        match self {
            SubscriptionTier::Free => 7,
            SubscriptionTier::Pro => 90,
            SubscriptionTier::Business => 365,
        }
    }

    pub fn has_custom_domains(&self) -> bool {
        matches!(self, SubscriptionTier::Pro | SubscriptionTier::Business)
    }

    pub fn has_api_access(&self) -> bool {
        matches!(self, SubscriptionTier::Pro | SubscriptionTier::Business)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    pub id: String,
    pub short_code: String,
    pub original_url: String,
    pub user_id: Option<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub clicks: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Click {
    pub id: String,
    pub url_id: String,
    pub clicked_at: String,
    pub ip_address: String,
    pub user_agent: String,
    pub referer: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    pub custom_alias: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
    pub short_code: String,
    pub original_url: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub url: Url,
    pub total_clicks: i32,
    pub clicks_by_date: Vec<DateCount>,
    pub clicks_by_country: Vec<CountryCount>,
    pub clicks_by_referer: Vec<RefererCount>,
}

#[derive(Debug, Serialize)]
pub struct DateCount {
    pub date: String,
    pub count: i32,
}

#[derive(Debug, Serialize)]
pub struct CountryCount {
    pub country: String,
    pub count: i32,
}

#[derive(Debug, Serialize)]
pub struct RefererCount {
    pub referer: String,
    pub count: i32,
}
