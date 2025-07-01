use crate::common::errors::DatabaseError;
use crate::routes::response::AppState;

pub struct DbMessage {
    message_id: u64,
    message: String,
}

impl From<tokio_postgres::Error> for DatabaseError {
    fn from(error: tokio_postgres::Error) -> DatabaseError {
        DatabaseError::Postgres(error)
    }
}

impl DbMessage {
    pub async fn insert_message(
        app_state: AppState,
        message: DbMessage,
    ) -> Result<(), DatabaseError> {
        let connection = app_state.db_pool.get().await.unwrap();

        connection
            .execute(
                "insert into message (message) values ($1)",
                &[&message.message],
            )
            .await?;

        Ok(())
    }

    pub async fn pop_message(app_state: AppState) {
        let connection = app_state.db_pool.get().await.unwrap();

        // connection.query("");
        todo!()
    }
}
