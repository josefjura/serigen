use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckUserPasswordError {
    #[error("User with password not found")]
    NotValid,

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}
