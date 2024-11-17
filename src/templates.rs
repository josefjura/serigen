use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;

use crate::db::Code;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub codes: Vec<Code>,
    pub from_protected: bool,
}

impl WithLayout for IndexTemplate {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub from_protected: bool,
}

impl WithLayout for LoginTemplate {}

#[derive(Template)]
#[template(path = "number.html")]
pub struct NumberTemplate {
    pub code: Code,
}

/// Error 401 page template
#[derive(Template)]
#[template(path = "401.html")]
pub struct Error401Template {
    pub reason: String,
    pub from_protected: bool,
}

impl WithLayout for Error401Template {}

pub trait WithLayout {
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

/// Struct for holding data from the user login form.
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}

//a wrapper for turning askama templates into responses that can be handled by server
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(), // If rendering is successful, return an HTML response
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR, // If rendering fails, return an internal server error
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
