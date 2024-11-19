use crate::{
    errors::add_number::AddNumberError,
    models::User,
    templates::{auth::ChangePasswordTemplate, codes::NumberTemplate, HtmlTemplate},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use chrono::{DateTime, Local};
use tower_sessions::Session;

use crate::{db::create_number, middleware::FROM_PROTECTED_KEY, state::AppState};

pub async fn add_code(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Response, Response> {
    let current_local: DateTime<Local> = Local::now();
    let code = current_local.format("V%Y%m%d").to_string();

    // Create the new number
    let created_code = create_number(&state.db, &code, &user.id.to_string()).await;

    match created_code {
        Err(AddNumberError::DuplicateCode(e)) => {
            Err((StatusCode::CONFLICT, format!("Duplicate code: {}", e)).into_response())
        }
        Err(AddNumberError::DbError(e)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create number: {}", e),
        )
            .into_response()),
        Err(AddNumberError::ParseSuffixError(e)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to parse the suffix: {}", e),
        )
            .into_response()),
        Ok(code) => Ok(HtmlTemplate(NumberTemplate { code }).into_response()),
    }
}

pub async fn change_password(
    session: Session,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    HtmlTemplate(ChangePasswordTemplate {
        from_protected,
        is_admin: user.is_admin,
        error: None,
    })
}
