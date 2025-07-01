use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool,
}

pub enum ApiResponse {
    Ok,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok => StatusCode::OK.into_response(),
        }
    }
}
