use crate::core::axum::AppState;
use crate::core::errors::*;
use std::sync::Arc;

pub struct DbMessage {
    message_id: u64,
    message: String,
}

impl DbMessage {
    pub async fn insert_message(
        app_state: Arc<AppState>,
        message: String,
    ) -> Result<(), DatabaseError> {
        let connection = app_state.db_pool.get().await?;

        connection
            .execute("insert into message (message) values ($1)", &[&message])
            .await?;

        Ok(())
    }

    pub async fn pop_message(app_state: AppState) -> Result<String, DatabaseError> {
        let connection = app_state.db_pool.get().await?;

        // connection.query("");
        todo!()
    }
}
