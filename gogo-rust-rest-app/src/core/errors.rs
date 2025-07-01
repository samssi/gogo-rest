use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::{CreatePoolError, PoolError};
use std::{fmt, io};

#[derive(Debug)]
pub enum ApplicationError {
    StartupError(String),
}

impl From<io::Error> for ApplicationError {
    fn from(err: io::Error) -> Self {
        ApplicationError::StartupError(err.to_string())
    }
}

impl From<CreatePoolError> for ApplicationError {
    fn from(err: CreatePoolError) -> Self {
        ApplicationError::StartupError(format!("Failed to create Postgres database pool: {err}"))
    }
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
