use crate::{
    db::read_all_users,
    errors::delete_user::DeleteUserError,
    forms::CreateUserSchema,
    middleware::FROM_PROTECTED_KEY,
    models::User,
    templates::{
        admin::{UserManagementTemplate, UserTemplate},
        errors::Error500Template,
        HtmlTemplate,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Form,
};
use tower_sessions::Session;

use crate::state::AppState;

pub async fn get_users(
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
            logged_user: Some(user.name.clone()),
        })
        .into_response()
    })?;

    Ok(HtmlTemplate(UserManagementTemplate {
        from_protected,
        is_admin: user.is_admin,
        logged_user: Some(user.name.clone()),
        users,
    })
    .into_response())
}

pub async fn delete_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Response, Response> {
    let result = crate::db::delete_user(&state.db, id as i64).await;

    match result {
        Ok(_) => Ok(().into_response()),
        Err(DeleteUserError::CantDeleteLastAdmin) => {
            Err((StatusCode::BAD_REQUEST, "Can't delete last admin").into_response())
        }
        Err(DeleteUserError::DbError(e)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )
            .into_response()),
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Form(user): Form<CreateUserSchema>,
) -> Result<Response, Response> {
    let user = crate::db::create_user(&state.db, user.name, user.password, user.is_admin).await;

    match user {
        Ok(user) => Ok(HtmlTemplate(UserTemplate { user }).into_response()),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {}", e),
        )
            .into_response()),
    }
}
