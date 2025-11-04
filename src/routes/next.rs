use crate::config::{RPD_LIMIT, RPM_LIMIT, TPM_LIMIT};
use crate::models::ApiKeyStatus;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct NextQuery {
    mode: Option<String>,
}

#[derive(Serialize)]
pub struct ApiKeyResponse {
    api_key: String,
}

pub async fn get_next_key(
    State(state): State<Arc<AppState>>,
    Query(query): Query<NextQuery>,
) -> Result<Json<ApiKeyResponse>, StatusCode> {
    let mode = query.mode.as_deref().unwrap_or("auto");

    // Check global rate limit
    let total_requests = state.total_requests_this_minute.read().unwrap();
    if *total_requests >= TPM_LIMIT {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let available_keys: Vec<_> = state
        .keys
        .iter()
        .filter(|entry| {
            let key = entry.value();
            key.status == ApiKeyStatus::Active
                && key.usage.requests_this_minute < RPM_LIMIT
                && key.usage.requests_this_day < RPD_LIMIT
        })
        .map(|entry| entry.key().clone())
        .collect();

    if available_keys.is_empty() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let selected_key_id = match mode {
        "random" => available_keys.choose(&mut rand::thread_rng()).cloned(),
        "auto" | _ => {
            let mut keys_with_last_used: Vec<_> = available_keys
                .iter()
                .map(|key_id| {
                    let key = state.keys.get(key_id).unwrap();
                    (key_id.clone(), key.last_used)
                })
                .collect();
            keys_with_last_used.sort_by_key(|&(_, last_used)| last_used);
            keys_with_last_used.first().map(|(key_id, _)| key_id.clone())
        }
    };

    if let Some(key_id) = selected_key_id {
        if let Some(mut key) = state.keys.get_mut(&key_id) {
            let mut total_requests = state.total_requests_this_minute.write().unwrap();
            *total_requests += 1;
            key.usage.requests_this_minute += 1;
            key.usage.requests_this_day += 1;
            key.last_used = Utc::now();
            return Ok(Json(ApiKeyResponse {
                api_key: key.key.clone(),
            }));
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
