use api_key_rotator::models::ApiKey;
use api_key_rotator::routes::create_router;
use api_key_rotator::state::AppState;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_next_key_no_keys() {
    let state = Arc::new(AppState::new());
    let router = create_router(state);

    let request = Request::builder()
        .uri("/next?mode=auto")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn test_add_key() {
    let state = Arc::new(AppState::new());
    let router = create_router(state.clone());

    let key = "test-key".to_string();
    let payload = json!({ "key": key });

    let request = Request::builder()
        .method(Method::POST)
        .uri("/add")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(state.keys.contains_key(&key));
}

#[tokio::test]
async fn test_deactivate_and_reactivate_key() {
    let state = Arc::new(AppState::new());
    let router = create_router(state.clone());
    let key = "test-key".to_string();
    state.keys.insert(key.clone(), ApiKey::new(key.clone()));

    // Deactivate the key
    let request = Request::builder()
        .method(Method::POST)
        .uri(format!("/deactivate/{}", key))
        .body(Body::empty())
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(state.keys.get(&key).unwrap().status, api_key_rotator::models::ApiKeyStatus::Inactive);

    // Reactivate the key
    let request = Request::builder()
        .method(Method::POST)
        .uri(format!("/reactivate/{}", key))
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(state.keys.get(&key).unwrap().status, api_key_rotator::models::ApiKeyStatus::Active);
}
