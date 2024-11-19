use askama::Template;

use super::WithLayout;

#[derive(Template)]
#[template(path = "pages/login/page.html")]
pub struct LoginPageTemplate {
    pub from_protected: bool,
    pub is_admin: bool,
    pub username: String,
    pub password: String,
    pub error: Option<String>,
}

impl WithLayout for LoginPageTemplate {}

#[derive(Template)]
#[template(path = "pages/login/section.html")]
pub struct LoginSectionTemplate {
    pub username: String,
    pub password: String,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/password_change/page.html")]
pub struct ChangePasswordPageTemplate {
    pub from_protected: bool,
    pub is_admin: bool,
    pub error: Option<String>,
}

impl WithLayout for ChangePasswordPageTemplate {}

#[derive(Template)]
#[template(path = "pages/password_change/section.html")]
pub struct ChangePasswordSectionTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/password_change/success.html")]
pub struct ChangePasswordSuccessTemplate {
    pub is_admin: bool,
    pub from_protected: bool,
}

impl WithLayout for ChangePasswordSuccessTemplate {}
