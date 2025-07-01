use crate::db::db::init_deadpool;
use crate::health::health::health_check;
use crate::messages::messages::{create_message, read_message};
use crate::routes::response::AppState;
use axum::routing::get;
use axum::Router;

pub async fn router() -> Router {
    let state = AppState {
        db_pool: init_deadpool(),
    };

    Router::new()
        .route("/health", get(health_check))
        .route("/v1/messages", get(read_message).post(create_message))
        .with_state(state)
}
