use crate::messages::messages::Message;
use crate::routes::response::AppState;

pub struct DbMessage {
    message_id: u64,
    message: String,
}

/*
pub async fn pop_message(app_state: AppState) {
    let connection = app_state.db_pool.get().await.unwrap();

    connection.query("")
}*/

pub async fn insert_message(app_state: AppState, message: Message) {
    let connection = app_state.db_pool.get().await.unwrap();

    connection
        .execute(
            "insert into message (message) values ($1)",
            &[&message.message],
        )
        .await
        .unwrap();
}
