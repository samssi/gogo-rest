use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::messages::messages::Message;

pub enum ApiResponse {
    Ok,
    JsonData(Message)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok => (StatusCode::OK).into_response(),
            ApiResponse::JsonData(message) => (StatusCode::OK, Json(message)).into_response()
        }
    }
}