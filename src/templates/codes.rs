use askama::Template;

use crate::models::Code;

use super::WithLayout;

#[derive(Template)]
#[template(path = "pages/index/page.html")]
pub struct IndexPageTemplate {
    pub codes: Vec<Code>,
    pub from_protected: bool,
    pub logged_user: Option<String>,
    pub is_admin: bool,
}

impl WithLayout for IndexPageTemplate {}

#[derive(Template)]
#[template(path = "pages/index/section.html")]
pub struct IndexSectionTemplate {
    pub codes: Vec<Code>,
}

#[derive(Template)]
#[template(path = "pages/index/code_item.html")]
pub struct CodeItemTemplate {
    pub code: Code,
}
