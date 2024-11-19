use askama::Template;

use crate::models::Code;

use super::WithLayout;

#[derive(Template)]
#[template(path = "pages/index/page.html")]
pub struct IndexTemplate {
    pub codes: Vec<Code>,
    pub from_protected: bool,
    pub is_admin: bool,
}

impl WithLayout for IndexTemplate {}

#[derive(Template)]
#[template(path = "pages/index/number.html")]
pub struct NumberTemplate {
    pub code: Code,
}
