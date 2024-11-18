use askama::Template;

use super::WithLayout;

/// Error 401 page template
#[derive(Template)]
#[template(path = "errors/401.html")]
pub struct Error401Template {
    pub reason: String,
    pub from_protected: bool,
}

impl WithLayout for Error401Template {}
