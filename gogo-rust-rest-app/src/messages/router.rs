use crate::core::axum::{ApiResponse, AppState};
use crate::core::errors::AxumApplicationError;
use crate::messages::service;
use crate::messages::service::MessageServiceError;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn create_messages_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/messages", post(Message::post_message))
        .route("/api/v1/messages", get(Message::get_message))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message: String,
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

impl Message {
    pub async fn post_message(
        State(app_state): State<Arc<AppState>>,
        Json(message): Json<Message>,
    ) -> Result<ApiResponse, AxumApplicationError> {
        println!("{:?}", message);

        service::Message::add_message(app_state, message.message).await?;

        Ok(ApiResponse::Ok)
    }

    pub async fn get_message(
        State(app_state): State<Arc<AppState>>,
    ) -> Result<ApiResponse, AxumApplicationError> {
        let _message = service::Message::read_message().await?;

        // TODO: return the message instead of OK later
        Ok(ApiResponse::Ok)
    }
}
