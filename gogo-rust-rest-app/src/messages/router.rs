use crate::common::axum::{ApiResponse, AppState};
use crate::common::errors::AxumApplicationError;
use crate::messages::service;
use crate::messages::service::MessageServiceError;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn create_messages_router() -> Router<Arc<AppState>> {
    Router::new().route("/api/v1/messages", post(create_message))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message: String,
}

pub async fn read_message(State(_app_state): State<AppState>) -> ApiResponse {
    let message = Message {
        message: String::from("Hi there!"),
    };

    // TODO: return the message instead of OK later
    ApiResponse::Ok
}

impl From<MessageServiceError> for AxumApplicationError {
    fn from(error: MessageServiceError) -> Self {
        match error {
            MessageServiceError::QueryError(error) => {
                AxumApplicationError::Internal(error.to_string())
            }
        }
    }
}

pub async fn create_message(
    State(app_state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Result<ApiResponse, AxumApplicationError> {
    println!("{:?}", message);

    service::Message::add_message(app_state, message.message).await?;

    Ok(ApiResponse::Ok)
}
