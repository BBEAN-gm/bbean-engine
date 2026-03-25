pub mod handlers;
pub mod middleware;
pub mod response;
pub mod state;

use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use state::AppState;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health))
        .route("/status", get(handlers::engine_status))
        .route("/tasks", post(handlers::submit_task))
        .route("/tasks/{task_id}", get(handlers::get_task))
        .route("/tasks/{task_id}/status", get(handlers::get_task_status))
        .route("/nodes", get(handlers::list_nodes))
        .route("/nodes/{node_id}", get(handlers::get_node))
        .route("/nodes/{node_id}/metrics", get(handlers::get_node_metrics))
        .route("/proofs/validate", post(handlers::validate_proof))
        .route("/config", get(handlers::get_config))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

pub async fn serve(host: &str, port: u16, state: Arc<AppState>) -> anyhow::Result<()> {
    let app = create_router(state);
    let addr = format!("{}:{}", host, port);
    tracing::info!("API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
