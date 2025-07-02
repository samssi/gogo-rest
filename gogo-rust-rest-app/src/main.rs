use crate::core::axum::AppState;
use crate::core::db::init_deadpool;
use crate::core::errors::ApplicationError;
use crate::health::router::create_health_router;
use crate::messages::router::create_messages_router;
use std::sync::Arc;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

mod core;
mod health;
mod messages;

fn create_state() -> Result<Arc<AppState>, ApplicationError> {
    let db_pool = init_deadpool()?;

    let state = Arc::new(AppState { db_pool });
    Ok(state)
}

async fn run_axum(
    listener_address: &str,
    state: Arc<AppState>,
    cancellation_token: CancellationToken,
) -> Result<(), ApplicationError> {
    let listener = tokio::net::TcpListener::bind(listener_address).await?;

    let routes = create_health_router()
        .merge(create_messages_router())
        .with_state(state);

    println!("Axum application listening on {}", listener_address);
    tokio::select! {
        result = axum::serve(listener, routes) => {
            result.map_err(|e| ApplicationError::StartupError(e.to_string()))
        }
        _ = cancellation_token.cancelled() => {
            println!("Axum server received cancellation signal");
            Ok(())
        }
    }
}

async fn run_tonic(
    listener_address: &str,
    state: Arc<AppState>,
    cancellation_token: CancellationToken,
) -> Result<(), ApplicationError> {
    println!("Tonic server will listen on {}", listener_address);

    tokio::select! {
        _ = async {
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;
                println!("dummy tonic is running...");
            }
        } => Ok(()), // This branch never ends unless cancelled

        _ = cancellation_token.cancelled() => {
            println!("Tonic server received cancellation signal");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let state = create_state()?;
    let axum_state = state.clone();
    let cancellation_token = CancellationToken::new();

    let axum_cancellation_token = cancellation_token.clone();
    let tonic_cancellation_token = cancellation_token.clone();

    let axum_task =
        tokio::spawn(
            async move { run_axum("0.0.0.0:3000", axum_state, axum_cancellation_token).await },
        );

    let tonic_task =
        tokio::spawn(
            async move { run_tonic("0.0.0.0:3001", state, tonic_cancellation_token).await },
        );

    axum_task
        .await
        .map_err(|e| ApplicationError::StartupError(format!("Axum task failed to join: {e}")))?
}
