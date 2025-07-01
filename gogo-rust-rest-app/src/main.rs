use crate::common::errors::ApplicationError;
use crate::routes::routes::router;

mod common;
mod db;
mod health;
mod messages;
mod routes;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let router = router().await;
    axum::serve(listener, router)
        .await
        .map_err(|e| ApplicationError::StartupError(e.to_string()))?;

    Ok(())
}
