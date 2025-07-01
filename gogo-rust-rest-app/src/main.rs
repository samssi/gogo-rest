use crate::routes::routes::router;

mod db;
mod health;
mod messages;
mod routes;

#[derive(Debug)]
pub enum ApplicationError {
    StartupError(String),
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let router = router().await;
    axum::serve(listener, router)
        .await
        .map_err(|e| ApplicationError::StartupError(e.to_string()))?;

    Ok(())
}
