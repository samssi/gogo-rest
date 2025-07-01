use crate::core::axum::AppState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;

pub fn create_health_router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    let payload = json!({ "status": "Ok" });
    (StatusCode::OK, Json(payload))
}
