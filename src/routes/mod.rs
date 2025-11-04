use crate::state::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

pub mod keys;
pub mod next;
pub mod stats;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/next", get(next::get_next_key))
        .route("/add", post(keys::add_key))
        .route("/add_bulk", post(keys::add_bulk_keys))
        .route("/delete/:key", delete(keys::delete_key))
        .route("/deactivate/:key", post(keys::deactivate_key))
        .route("/reactivate/:key", post(keys::reactivate_key))
        .route("/keys", get(keys::get_all_keys))
        .route("/stats", get(stats::get_stats))
        .with_state(state)
}
