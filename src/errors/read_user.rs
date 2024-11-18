use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadUserError {
    #[error("User with ID: '{0}' not found")]
    UserNotFound(String),

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}

// Implement IntoResponse for AddNumberError
impl IntoResponse for ReadUserError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            ReadUserError::UserNotFound(user_id) => (
                StatusCode::NOT_FOUND,
                format!("User with ID: {} does not exist", user_id),
            ),
            ReadUserError::DbError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, error_message).into_response()
    }
}
