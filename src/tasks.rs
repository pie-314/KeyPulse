use crate::config::KEY_COOLDOWN_SECONDS;
use crate::models::ApiKeyStatus;
use crate::state::AppState;
use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio::time::{self, Duration as TokioDuration};

pub fn spawn_tasks(state: Arc<AppState>) {
    // Task to save keys to keys.json every 15 seconds
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(TokioDuration::from_secs(15));
        loop {
            interval.tick().await;
            if let Err(e) = state_clone.save() {
                eprintln!("Failed to save keys: {}", e);
            }
        }
    });

    // Task to reset minute counters every 60 seconds
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(TokioDuration::from_secs(60));
        loop {
            interval.tick().await;
            for mut entry in state_clone.keys.iter_mut() {
                entry.value_mut().usage.requests_this_minute = 0;
            }
        }
    });

    // Task to reset daily counters every 24 hours
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(TokioDuration::from_secs(24 * 60 * 60));
        loop {
            interval.tick().await;
            for mut entry in state_clone.keys.iter_mut() {
                entry.value_mut().usage.requests_this_day = 0;
            }
        }
    });

    // Task to reactivate inactive keys after cooldown
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(TokioDuration::from_secs(60)); // Check every minute
        loop {
            interval.tick().await;
            for mut entry in state_clone.keys.iter_mut() {
                let key = entry.value_mut();
                if key.status == ApiKeyStatus::Inactive {
                    if let Some(deactivated_at) = key.deactivated_at {
                        let cooldown = Duration::seconds(KEY_COOLDOWN_SECONDS);
                        if Utc::now().signed_duration_since(deactivated_at) > cooldown {
                            key.status = ApiKeyStatus::Active;
                            key.deactivated_at = None;
                        }
                    }
                }
            }
        }
    });

    // Task to reset global request counter every 60 seconds
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(TokioDuration::from_secs(60));
        loop {
            interval.tick().await;
            let mut total_requests = state_clone.total_requests_this_minute.write().unwrap();
            *total_requests = 0;
        }
    });
}
