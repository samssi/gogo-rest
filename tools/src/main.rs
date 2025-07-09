use crate::proto_gen::buf::Buf;
use tokio::signal;
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::oneshot;
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
        println!("buf code generation completed successfully");
        Ok(())
    } else {
        Err(ApplicationError::ExecutionError(
            "buf code generation failed".to_string(),
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
    let (result_tx, result_rx) = oneshot::channel();

    let task_tracker = TaskTracker::new();
    task_tracker.spawn(async move {
        let result = buf_task(run_buf_cancellation_token).await;
        let _ = result_tx.send(result);
    });

    tokio::select! {
        result = result_rx => {
            cancel_tasks(task_tracker, cancellation_token).await;
            match result {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => Err(e),
                Err(_) => Err(ApplicationError::ExecutionError("buf task channel dropped".into())),
            }
        }
        _ = shutdown_signal() => {
            cancel_tasks(task_tracker, cancellation_token).await;
            Ok(())
        }
    }
}
