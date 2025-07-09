use crate::core::db::init_deadpool;
use crate::core::errors::ApplicationError;
use crate::core::state::AppState;
use crate::core::tonic::Tonic;
use crate::health::router::create_health_router;
use crate::messages::router::create_messages_router;
use std::sync::Arc;
use std::time::Instant;
use tokio::signal;
use tokio::signal::unix::{SignalKind, signal};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

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

    println!("Axum application listening on {listener_address}");
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
    tonic_address: &str,
    state: Arc<AppState>,
    cancellation_token: CancellationToken,
) -> Result<(), ApplicationError> {
    println!("Tonic server will listen on {tonic_address}");

    tokio::select! {
        result = Tonic::run(state.clone(), tonic_address) => {
            result
        }
        _ = cancellation_token.cancelled() => {
            println!("Tonic server received cancellation signal");
            Ok(())
        }
    }
}

async fn shutdown_signal() -> Result<(), ApplicationError> {
    let mut sigterm = signal(SignalKind::terminate())
        .map_err(|err| ApplicationError::StartupError(err.to_string()))?;
    let mut sigint = signal(SignalKind::interrupt())
        .map_err(|err| ApplicationError::StartupError(err.to_string()))?;

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C");
        }
        _ = sigterm.recv() => {
            println!("Received SIGTERM");
        }
        _ = sigint.recv() => {
            println!("Received SIGINT");
        }
    }

    Ok(())
}

async fn cancel_tasks(task_tracker: TaskTracker, cancellation_token: CancellationToken) {
    let start_time = Instant::now();
    println!("Shutdown initiated, waiting for background tasks to complete...");
    task_tracker.close();
    cancellation_token.cancel();
    task_tracker.wait().await;

    let duration = start_time.elapsed();
    println!("Shutdown in {duration:?}");
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let state = create_state()?;
    let axum_state = state.clone();
    let cancellation_token = CancellationToken::new();

    let axum_cancellation_token = cancellation_token.clone();
    let tonic_cancellation_token = cancellation_token.clone();

    let task_tracker = TaskTracker::new();

    task_tracker
        .spawn(async move { run_axum("0.0.0.0:3000", axum_state, axum_cancellation_token).await });

    task_tracker
        .spawn(async move { run_tonic("0.0.0.0:3001", state, tonic_cancellation_token).await });

    shutdown_signal().await?;
    cancel_tasks(task_tracker, cancellation_token).await;

    Ok(())
}
