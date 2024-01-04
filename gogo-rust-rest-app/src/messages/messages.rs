use serde::Serialize;
use crate::routes::response::ApiResponse;

#[derive(Serialize)]
pub struct Message {
    pub message: String
}

pub async fn read_message() -> ApiResponse {
    let message = Message {
        message: String::from("Hi there!")
    };

    ApiResponse::JsonData(message)
}