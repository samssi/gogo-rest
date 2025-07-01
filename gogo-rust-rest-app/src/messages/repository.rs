use crate::core::axum::AppState;
use crate::core::errors::*;
use std::sync::Arc;

pub struct DbMessage {
    pub message_id: i32,
    pub message: String,
}

impl DbMessage {
    pub fn from_row(row: tokio_postgres::Row) -> Result<Self, DatabaseError> {
        Ok(Self {
            message_id: row.try_get("message_id")?,
            message: row.try_get("message")?,
        })
    }

    pub async fn insert_message(
        state: Arc<AppState>,
        message: String,
    ) -> Result<(), DatabaseError> {
        let connection = state.db_pool.get().await?;

        connection
            .execute("insert into message (message) values ($1)", &[&message])
            .await?;

        Ok(())
    }

    pub async fn pop_message(state: Arc<AppState>) -> Result<Option<DbMessage>, DatabaseError> {
        let connection = state.db_pool.get().await?;

        let row = connection
            .query_opt(
                r#"
                DELETE FROM message
                WHERE message_id = (
                    SELECT message_id
                    FROM message
                    ORDER BY message_id ASC
                    LIMIT 1
                )
                RETURNING message_id, message;
                "#,
                &[],
            )
            .await?;

        let message = row.map(DbMessage::from_row).transpose()?;
        Ok(message)
    }
}
