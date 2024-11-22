use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResetCodesError {
    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}
