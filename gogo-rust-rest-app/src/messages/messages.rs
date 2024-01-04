use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::messages::db::insert_message;
use crate::routes::response::{ApiResponse, AppState};

#[derive(Serialize,Deserialize,Debug)]
pub struct Message {
    pub message: String
}

pub async fn read_message(State(_app_state): State<AppState>) -> ApiResponse {

    let message = Message {
        message: String::from("Hi there!")
    };

    ApiResponse::JsonData(message)
}

pub async fn create_message(State(app_state): State<AppState>, Json(message): Json<Message>) -> ApiResponse {
    println!("{:?}", message);
    insert_message(app_state, message).await;

    ApiResponse::Ok
}