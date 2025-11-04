use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ApiKeyStatus {
    Active,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyUsage {
    pub requests_this_minute: u32,
    pub requests_this_day: u32,
}

impl Default for ApiKeyUsage {
    fn default() -> Self {
        Self {
            requests_this_minute: 0,
            requests_this_day: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub key: String,
    pub status: ApiKeyStatus,
    pub usage: ApiKeyUsage,
    pub last_used: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub deactivated_at: Option<DateTime<Utc>>,
}

impl ApiKey {
    pub fn new(key: String) -> Self {
        Self {
            key,
            status: ApiKeyStatus::Active,
            usage: ApiKeyUsage::default(),
            last_used: Utc::now(),
            created_at: Utc::now(),
            deactivated_at: None,
        }
    }
}
