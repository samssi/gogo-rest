use crate::messages::messages::create_message;
use crate::routes::response::AppState;
use axum::routing::post;
use axum::Router;

pub fn create_messages_router() -> Router<AppState> {
    Router::new().route("/api/v1/messages", post(create_message))
}
