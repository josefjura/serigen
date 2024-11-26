use askama::Template;

use crate::models::User;

use super::WithLayout;

#[derive(Template)]
#[template(path = "pages/user_management/page.html")]
pub struct UserManagementTemplate {
    pub from_protected: bool,
    pub is_admin: bool,
    pub logged_user: Option<String>,
    pub users: Vec<User>,
}

impl WithLayout for UserManagementTemplate {}

#[derive(Template)]
#[template(path = "pages/user_management/user.html")]
pub struct UserTemplate {
    pub user: User,
}
