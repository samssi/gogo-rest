use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    StartupError(String),
}

pub enum DatabaseError {
    Postgres(tokio_postgres::Error),
}

pub enum AxumApplicationError {
    BadRequest(String),
    NotFound(String),
    Internal(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::Postgres(err) => write!(f, "Postgres error: {}", err),
        }
    }
}

impl IntoResponse for AxumApplicationError {
    fn into_response(self) -> Response {
        match self {
            AxumApplicationError::BadRequest(error) => {
                (StatusCode::BAD_REQUEST, error).into_response()
            }
            AxumApplicationError::NotFound(error) => (StatusCode::NOT_FOUND, error).into_response(),
            AxumApplicationError::Internal(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error).into_response()
            }
        }
    }
}
