use std::env::VarError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Cannot bind to selected address. Error: {0}")]
    CannotBind(#[from] std::io::Error),
    #[error("Error while starting the server. Error: {0}")]
    CannotServe(std::io::Error),
    #[error("Error while connecting to the database. Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Missing environment value: {1}")]
    EnvError(#[source] VarError, String),
}

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
