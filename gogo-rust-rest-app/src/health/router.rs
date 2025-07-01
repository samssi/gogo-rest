use crate::core::axum::{ApiResponse, AppState};
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn create_health_router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check() -> ApiResponse {
    ApiResponse::Ok
}
