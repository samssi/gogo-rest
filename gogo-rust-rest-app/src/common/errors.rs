#[derive(Debug)]
pub enum ApplicationError {
    StartupError(String),
}

pub enum DatabaseError {
    Postgres(tokio_postgres::Error),
}
