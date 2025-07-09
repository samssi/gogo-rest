use crate::core::errors::DatabaseError;
use crate::core::state::AppState;
use crate::messages::repository::DbMessage;
use std::sync::Arc;

pub struct Message {
    pub message: String,
}

pub enum MessageServiceError {
    QueryError(DatabaseError),
}

impl From<DatabaseError> for MessageServiceError {
    fn from(error: DatabaseError) -> Self {
        MessageServiceError::QueryError(error)
    }
}

impl Message {
    pub async fn add_message(
        state: Arc<AppState>,
        message: String,
    ) -> Result<(), MessageServiceError> {
        DbMessage::insert_message(state, message).await?;
        Ok(())
    }

    pub async fn read_message(state: Arc<AppState>) -> Result<Option<String>, MessageServiceError> {
        let db_message = DbMessage::pop_message(state).await?;
        let message = match db_message {
            Some(db_message) => Some(db_message.message),
            None => None,
        };

        Ok(message)
    }
}
