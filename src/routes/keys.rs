use crate::models::{ApiKey, ApiKeyStatus};
use crate::state::AppState;
use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::Json as JsonResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AddKeyPayload {
    key: String,
}

pub async fn add_key(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AddKeyPayload>,
) -> StatusCode {
    let new_key = ApiKey::new(payload.key.clone());
    state.keys.insert(payload.key, new_key);
    StatusCode::CREATED
}

#[derive(Deserialize)]
pub struct BulkAddKeysPayload {
    keys: Vec<String>,
}

pub async fn add_bulk_keys(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BulkAddKeysPayload>,
) -> StatusCode {
    for key_str in payload.keys {
        let new_key = ApiKey::new(key_str.clone());
        state.keys.insert(key_str, new_key);
    }
    StatusCode::CREATED
}

pub async fn delete_key(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> StatusCode {
    if state.keys.remove(&key).is_some() {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn deactivate_key(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> StatusCode {
    if let Some(mut api_key) = state.keys.get_mut(&key) {
        api_key.status = ApiKeyStatus::Inactive;
        api_key.deactivated_at = Some(Utc::now());
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn reactivate_key(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> StatusCode {
    if let Some(mut api_key) = state.keys.get_mut(&key) {
        api_key.status = ApiKeyStatus::Active;
        api_key.deactivated_at = None;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

#[derive(Serialize)]
pub struct AllKeysResponse {
    keys: Vec<ApiKey>,
}

pub async fn get_all_keys(
    State(state): State<Arc<AppState>>,
) -> JsonResponse<AllKeysResponse> {
    let keys = state
        .keys
        .iter()
        .map(|entry| entry.value().clone())
        .collect();
    JsonResponse(AllKeysResponse { keys })
}
