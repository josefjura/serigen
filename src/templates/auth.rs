use askama::Template;

use super::WithLayout;

#[derive(Template)]
#[template(path = "pages/login/page.html")]
pub struct LoginTemplate {
    pub from_protected: bool,
}

impl WithLayout for LoginTemplate {}

#[derive(Template)]
#[template(path = "pages/password_change/page.html")]
pub struct PasswordChangeTemplate {
    pub from_protected: bool,
}

impl WithLayout for PasswordChangeTemplate {}

#[derive(Template)]
#[template(path = "pages/password_change/success.html")]
pub struct ChangePasswordSuccessTemplate {}
