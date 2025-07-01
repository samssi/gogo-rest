use crate::core::axum::AppState;
use crate::core::db::init_deadpool;
use crate::core::errors::ApplicationError;
use crate::health::router::create_health_router;
use crate::messages::router::create_messages_router;
use std::sync::Arc;

mod core;
mod health;
mod messages;

fn create_state() -> Result<Arc<AppState>, ApplicationError> {
    let db_pool = init_deadpool()?;

    let state = Arc::new(AppState { db_pool });
    Ok(state)
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let state = create_state()?;

    let routes = create_health_router()
        .merge(create_messages_router())
        .with_state(state);

    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, routes).await?;

    Ok(())
}
