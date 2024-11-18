use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordChangeError {
    #[error("Unable to authenticate user. Old password is incorrect.")]
    OldPasswordIsIncorrect,

    #[error("New password is the same as the old password")]
    PasswordIsSameAsOld,

    #[error("New password and retype password do not match")]
    PasswordsDontMatch,

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}

// Implement IntoResponse for AddNumberError
impl IntoResponse for PasswordChangeError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            PasswordChangeError::OldPasswordIsIncorrect => (
                StatusCode::UNAUTHORIZED,
                "Old password is incorrect".to_string(),
            ),
            PasswordChangeError::PasswordIsSameAsOld => (
                StatusCode::BAD_REQUEST,
                "New password is the same as the old password".to_string(),
            ),
            PasswordChangeError::PasswordsDontMatch => (
                StatusCode::BAD_REQUEST,
                "New password and retype password do not match".to_string(),
            ),
            PasswordChangeError::DbError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, error_message).into_response()
    }
}
