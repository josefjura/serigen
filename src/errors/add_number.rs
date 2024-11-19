use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddNumberError {
    #[error("Failed to parse suffix from the code: {0}")]
    ParseSuffixError(String),

    #[error("Code '{0}' already exists")]
    DuplicateCode(String),

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}
