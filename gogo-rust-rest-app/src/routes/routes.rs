use axum::{Router};
use axum::routing::get;
use crate::health::health::health_check;
use crate::messages::messages::{read_message, create_message};
pub fn router() -> Router {
    Router::new()
        .route("/health/", get(health_check))
        .route("/v1/messages/",
               get(read_message).post(create_message))
}