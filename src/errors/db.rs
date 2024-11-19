// use axum::response::{IntoResponse, Response};
// use thiserror::Error;

// use crate::templates::{errors::Error500Template, HtmlTemplate};

// #[derive(Debug, Error)]
// pub enum Http500 {
//     #[error("Error communicating with database: '{error}'")]
//     DbError {
//         #[source]
//         error: sqlx::Error,
//         from_protected: bool,
//         is_admin: bool,
//     },
// }

// impl IntoResponse for Http500 {
//     fn into_response(self) -> Response {
//         HtmlTemplate(Error500Template {
//             from_protected: false,
//             is_admin: false,
//             reason: self.to_string(),
//         })
//         .into_response()
//     }
// }
