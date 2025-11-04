use crate::config::KEY_COOLDOWN_SECONDS;
use crate::models::ApiKey;
use chrono::{Duration, Utc};

pub fn get_api_key_expiry(key: &ApiKey) -> i64 {
    if let Some(deactivated_at) = key.deactivated_at {
        let now = Utc::now();
        let time_since_deactivated = now.signed_duration_since(deactivated_at);
        let cooldown = Duration::seconds(KEY_COOLDOWN_SECONDS);
        let remaining_cooldown = cooldown - time_since_deactivated;

        if remaining_cooldown.num_seconds() > 0 {
            remaining_cooldown.num_seconds()
        } else {
            0
        }
    } else {
        0
    }
}
