use crate::common::axum::AppState;
use crate::common::db::init_deadpool;
use crate::common::errors::ApplicationError;
use crate::health::router::create_health_router;
use crate::messages::router::create_messages_router;
use std::sync::Arc;

mod common;
mod health;
mod messages;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let state = Arc::new(AppState {
        db_pool: init_deadpool().map_err(|e| {
            ApplicationError::StartupError(format!("Failed to create Postgres database pool: {e}"))
        })?,
    });

    let routes = create_health_router()
        .merge(create_messages_router())
        .with_state(state);

    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, routes)
        .await
        .map_err(|e| ApplicationError::StartupError(e.to_string()))?;

    Ok(())
}
