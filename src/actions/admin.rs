use crate::{
    db::read_all_users,
    middleware::FROM_PROTECTED_KEY,
    models::User,
    templates::{admin::UserManagementTemplate, errors::Error500Template, HtmlTemplate},
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension,
};
use tower_sessions::Session;

use crate::state::AppState;

pub async fn user_management(
    session: Session,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Response, Response> {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    let users = read_all_users(&state.db).await.map_err(|e| {
        HtmlTemplate(Error500Template {
            from_protected,
            is_admin: user.is_admin,
            reason: format!("Failed to read users: {}", e),
        })
        .into_response()
    })?;

    Ok(HtmlTemplate(UserManagementTemplate {
        from_protected,
        is_admin: user.is_admin,
        users,
    })
    .into_response())
}
