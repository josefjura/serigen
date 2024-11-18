use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckUserPasswordError {
    #[error("User with password not found")]
    NotValid,

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}

// Implement IntoResponse for AddNumberError
impl IntoResponse for CheckUserPasswordError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            CheckUserPasswordError::NotValid => {
                (StatusCode::UNAUTHORIZED, format!("User does not exist"))
            }
            CheckUserPasswordError::DbError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        };

        (status, error_message).into_response()
    }
}
