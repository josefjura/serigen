use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
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

// Implement IntoResponse for AddNumberError
impl IntoResponse for AddNumberError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AddNumberError::ParseSuffixError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Failed to parse suffix: {message}"),
            ),
            AddNumberError::DuplicateCode(_) => (
                StatusCode::CONFLICT,
                "Failed to create a new number".to_string(),
            ),
            AddNumberError::DbError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, error_message).into_response()
    }
}
