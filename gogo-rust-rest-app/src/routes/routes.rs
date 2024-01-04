use axum::{Router};
use axum::routing::get;
use deadpool_postgres::Pool;
use crate::health::health::health_check;
use crate::messages::messages::{read_message, create_message};
use crate::routes::response::AppState;

pub fn router(dbPool: Pool) -> Router {
    let state = AppState {
        db_pool: dbPool
    };

    Router::new()
        .route("/health/", get(health_check))
        .route("/v1/messages/", get(read_message).post(create_message))
        .with_state(state)
}