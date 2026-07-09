use askama::Template;

use crate::models::Code;

#[derive(Template)]
#[template(path = "pages/index/code_item.html")]
pub struct CodeItemTemplate {
    pub code: Code,
}
