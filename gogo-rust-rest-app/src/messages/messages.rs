use axum::Json;
use serde::{Deserialize, Serialize};
use crate::routes::response::ApiResponse;

#[derive(Serialize,Deserialize,Debug)]
pub struct Message {
    pub message: String
}

pub async fn read_message() -> ApiResponse {
    let message = Message {
        message: String::from("Hi there!")
    };

    ApiResponse::JsonData(message)
}

pub async fn create_message(Json(message): Json<Message>) -> ApiResponse {
    println!("{:?}", message);

    ApiResponse::Ok
}