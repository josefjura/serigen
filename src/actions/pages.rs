use crate::{
    models::User,
    templates::{codes::IndexPageTemplate, HtmlTemplate},
};
use axum::{extract::State, response::IntoResponse, Extension};
use tower_sessions::Session;

use crate::{db::read_last_ten, middleware::FROM_PROTECTED_KEY, state::AppState};

pub async fn index(
    session: Session,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    let last_ten = read_last_ten(&state.db).await;

    if let Ok(last_ten) = last_ten {
        Ok(HtmlTemplate(IndexPageTemplate {
            codes: last_ten,
            from_protected,
            is_admin: user.is_admin,
            logged_user: Some(user.name.clone()),
        }))
    } else {
        Err("Failed to read last ten numbers")
    }
}
