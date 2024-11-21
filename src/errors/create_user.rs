use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("Created user can't be found")]
    CantRead,

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}
