use askama::Template;

use super::WithLayout;

#[derive(Template)]
#[template(path = "pages/login/page.html")]
pub struct LoginTemplate {
    pub from_protected: bool,
    pub is_admin: bool,
    pub username: String,
    pub password: String,
    pub error: Option<String>,
}

impl WithLayout for LoginTemplate {}

#[derive(Template)]
#[template(path = "pages/password_change/page.html")]
pub struct ChangePasswordTemplate {
    pub from_protected: bool,
    pub is_admin: bool,
    pub error: Option<String>,
}

impl WithLayout for ChangePasswordTemplate {}

#[derive(Template)]
#[template(path = "pages/password_change/success.html")]
pub struct ChangePasswordSuccessTemplate {
    pub is_admin: bool,
    pub from_protected: bool,
}

impl WithLayout for ChangePasswordSuccessTemplate {}
