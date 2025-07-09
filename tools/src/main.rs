use crate::proto_gen::buf::Buf;
use tokio::signal;
use tokio::signal::unix::{SignalKind, signal};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

mod proto_gen;
mod util;

#[derive(Debug)]
pub enum ApplicationError {
    ExecutionError(String),
}

pub async fn run_buf() -> Result<(), ApplicationError> {
    let buf = Buf::new().await;
    let buf_exec_status = buf.exec_buf().await;

    if buf_exec_status.success() {
        println!("Buf code generation completed successfully");
        Ok(())
    } else {
        Err(ApplicationError::ExecutionError(
            "buf code generation failed.".to_string(),
        ))
    }
}

pub async fn buf_task(cancellation_token: CancellationToken) -> Result<(), ApplicationError> {
    tokio::select! {
        result = run_buf() => result,
        _ = cancellation_token.cancelled() => {
            Ok(())
        }
    }
}

async fn shutdown_signal() -> Result<(), ApplicationError> {
    let mut sigterm = signal(SignalKind::terminate())
        .map_err(|err| ApplicationError::ExecutionError(err.to_string()))?;
    let mut sigint = signal(SignalKind::interrupt())
        .map_err(|err| ApplicationError::ExecutionError(err.to_string()))?;

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
    task_tracker.close();
    cancellation_token.cancel();
    task_tracker.wait().await;
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let cancellation_token = CancellationToken::new();
    let run_buf_cancellation_token = cancellation_token.clone();

    let task_tracker = TaskTracker::new();
    let buf_task_handle =
        task_tracker.spawn(async move { buf_task(run_buf_cancellation_token).await });

    tokio::select! {
        _ = buf_task_handle => {
            cancel_tasks(task_tracker, cancellation_token).await;
            Ok(())
        }
        _ = shutdown_signal() => {
            cancel_tasks(task_tracker, cancellation_token).await;
            Ok(())
        }
    }
}
