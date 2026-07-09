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
#[template(path = "pages/list/page.html")]
pub struct ListPageTemplate {
    pub codes: Vec<Code>,
		pub total: i64,
    pub from_protected: bool,
    pub logged_user: Option<String>,
    pub is_admin: bool,
    pub(crate) page: u32,
    pub(crate) per_page: u32,
    pub(crate) code_filter: String,
    pub(crate) user_filter: String,
    pub(crate) total_pages: u32,
    pub(crate) has_prev: bool,
    pub(crate) has_next: bool,
    pub(crate) prev_page: u32,
    pub(crate) next_page: u32,
}

impl WithLayout for ListPageTemplate {}

#[derive(Template)]
#[template(path = "pages/list/search_results.html")]
pub struct ListSectionTemplate {
    pub codes: Vec<Code>,
		pub total: i64,
		pub(crate) page: u32,
		pub(crate) per_page: u32,
		pub(crate) code_filter: String,
		pub(crate) user_filter: String,
		pub(crate) total_pages: u32,
		pub(crate) has_prev: bool,
		pub(crate) has_next: bool,
		pub(crate) prev_page: u32,
		pub(crate) next_page: u32,
}
