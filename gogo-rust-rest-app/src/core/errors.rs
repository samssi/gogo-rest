use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::PoolError;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    StartupError(String),
}

pub enum DatabaseError {
    Postgres(tokio_postgres::Error),
    PostgresPoolError(PoolError),
}

impl From<tokio_postgres::Error> for DatabaseError {
    fn from(error: tokio_postgres::Error) -> DatabaseError {
        DatabaseError::Postgres(error)
    }
}

impl From<PoolError> for DatabaseError {
    fn from(value: PoolError) -> Self {
        DatabaseError::PostgresPoolError(value)
    }
}

pub enum AxumApplicationError {
    BadRequest(String),
    NotFound(String),
    Internal(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::Postgres(error) => write!(f, "Postgres error: {}", error),
            DatabaseError::PostgresPoolError(error) => write!(f, "Postgres pool error: {}", error),
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
