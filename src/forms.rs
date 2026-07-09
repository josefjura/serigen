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

/// Struct for holding data from the change password form.
#[derive(Debug, Deserialize)]
pub struct CreateUserSchema {
    pub name: String,
    pub password: String,
    #[serde(default)]
    pub is_admin: bool,
}

/// Struct for accessing paged list of items.
#[derive(Debug, Deserialize)]
pub struct SearchItemsSchema {
    pub filter: Option<String>,
    pub page: Option<u32>,
    pub count: Option<u32>,
}
