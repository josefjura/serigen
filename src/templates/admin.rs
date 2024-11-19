use askama::Template;

use crate::models::User;

use super::WithLayout;

#[derive(Template)]
#[template(path = "pages/user_management/page.html")]
pub struct UserManagementTemplate {
    pub from_protected: bool,
    pub is_admin: bool,
    pub users: Vec<User>,
}

impl WithLayout for UserManagementTemplate {}
