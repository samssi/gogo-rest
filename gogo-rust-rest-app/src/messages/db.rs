use crate::messages::messages::Message;
use crate::routes::response::AppState;

pub struct DbMessage {
    message_id: u64,
    message: String
}

pub async fn pop_message(app_state: AppState) {
    let connection = app_state.db_pool.get().await.unwrap();

    for i in 1..10 {
        let stmt = connection.prepare_cached("SELECT 1 + $1").await.unwrap();
        let rows = connection.query(&stmt, &[&i]).await.unwrap();
        let value: i32 = rows[0].get(0);
        assert_eq!(value, i + 1);
        println!("{:?}", value);
    }
}

pub async fn insert_message(app_state: AppState, message: Message) {
    let connection = app_state.db_pool.get().await.unwrap();

    connection.execute(
        "insert into message (message) values ($1)",
        &[&message.message]
    ).await.unwrap();
}