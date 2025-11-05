use api_key_rotator::routes;
use api_key_rotator::state::AppState;
use api_key_rotator::tasks::spawn_tasks;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::load());

    spawn_tasks(state.clone());

    let app = routes::create_router(state).layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
