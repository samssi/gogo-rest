use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::routes::response::{ApiResponse, AppState};

#[derive(Serialize,Deserialize,Debug)]
pub struct Message {
    pub message: String
}

pub async fn read_message(State(app_state): State<AppState>) -> ApiResponse {
    let connection = app_state.db_pool.get().await.unwrap();
    for i in 1..10 {
        let stmt = connection.prepare_cached("SELECT 1 + $1").await.unwrap();
        let rows = connection.query(&stmt, &[&i]).await.unwrap();
        let value: i32 = rows[0].get(0);
        assert_eq!(value, i + 1);
        println!("{:?}", value);
    }

    let message = Message {
        message: String::from("Hi there!")
    };

    ApiResponse::JsonData(message)
}

pub async fn create_message(Json(message): Json<Message>) -> ApiResponse {
    println!("{:?}", message);

    ApiResponse::Ok
}