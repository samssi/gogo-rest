use crate::common::errors::AxumApplicationError;
use crate::messages::service;
use crate::messages::service::MessageServiceError;
use crate::routes::response::{ApiResponse, AppState};
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message: String,
}

pub async fn read_message(State(_app_state): State<AppState>) -> ApiResponse {
    let message = Message {
        message: String::from("Hi there!"),
    };

    ApiResponse::JsonData(message)
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
    State(app_state): State<AppState>,
    Json(message): Json<Message>,
) -> Result<ApiResponse, AxumApplicationError> {
    println!("{:?}", message);

    service::Message::add_message(app_state, message.message).await?;

    Ok(ApiResponse::Ok)
}
