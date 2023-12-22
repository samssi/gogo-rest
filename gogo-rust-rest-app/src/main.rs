use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use crate::ApiResponse::JsonData;

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

fn router() -> Router {
    Router::new()
        .route("/", get(read_message))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}
