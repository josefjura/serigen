use serde::Deserialize;

/// Struct for holding data from the user login form.
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}

/// Struct for holding data from the change password form.
#[derive(Debug, Deserialize)]
pub struct ChangePasswordSchema {
    pub old_password: String,
    pub new_password: String,
    pub retype_password: String,
}
