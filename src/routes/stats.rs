use crate::models::ApiKeyStatus;
use crate::state::AppState;
use axum::{extract::State, response::Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct StatsResponse {
    total_keys: usize,
    active_keys: usize,
    inactive_keys: usize,
}

pub async fn get_stats(State(state): State<Arc<AppState>>) -> Json<StatsResponse> {
    let total_keys = state.keys.len();
    let active_keys = state
        .keys
        .iter()
        .filter(|entry| entry.value().status == ApiKeyStatus::Active)
        .count();
    let inactive_keys = total_keys - active_keys;

    Json(StatsResponse {
        total_keys,
        active_keys,
        inactive_keys,
    })
}
