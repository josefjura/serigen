use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadUserError {
    #[error("User with ID: '{0}' not found")]
    UserNotFound(String),

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}
