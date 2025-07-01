use crate::common::errors::DatabaseError;
use crate::messages::repository::DbMessage;
use crate::routes::response::AppState;

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
    pub async fn add_message(state: AppState, message: String) -> Result<(), MessageServiceError> {
        DbMessage::insert_message(state, message).await?;
        Ok(())
    }
}
