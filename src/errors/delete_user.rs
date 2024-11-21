use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeleteUserError {
    #[error("Can't delete the last admin user")]
    CantDeleteLastAdmin,

    #[error("Error communicating with database: '{0}'")]
    DbError(#[from] sqlx::Error),
}
