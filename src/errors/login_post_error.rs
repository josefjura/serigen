use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginPostError {
    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}
