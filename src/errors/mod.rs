use std::env::VarError;

use thiserror::Error;

pub mod add_number;
pub mod check_user_password;
pub mod create_user;
pub mod delete_user;
pub mod login_post_error;
pub mod password_change;
pub mod read_user;
pub mod read_users;

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
