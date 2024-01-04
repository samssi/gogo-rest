use axum::{Router};
use axum::routing::get;
use crate::db::db::init_deadpool;
use crate::health::health::health_check;
use crate::messages::messages::{read_message, create_message};
use crate::routes::response::AppState;

pub async fn router() -> Router {
    let state = AppState {
        db_pool: init_deadpool()
    };

    Router::new()
        .route("/health/", get(health_check))
        .route("/v1/messages/", get(read_message).post(create_message))
        .with_state(state)
}