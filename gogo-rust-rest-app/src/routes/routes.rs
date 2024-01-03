use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use crate::routes::routes::ApiResponse::JsonData;


#[derive(Serialize)]
struct Message {
    message: String
}

enum ApiResponse {
    Ok,
    JsonData(Message)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok => (StatusCode::OK).into_response(),
            JsonData(message) => (StatusCode::OK, Json(message)).into_response()
        }
    }
}

async fn read_message() -> ApiResponse {
    let message = Message {
        message: String::from("Hi there!")
    };

    JsonData(message)
}

async fn health_check() -> ApiResponse {
    ApiResponse::Ok
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/v1/messages", get(read_message))
    // .route("/v1/messages", post(post_message))
}