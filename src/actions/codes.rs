use crate::{
    errors::add_number::AddNumberError,
    models::User,
    templates::{auth::PasswordChangeTemplate, codes::NumberTemplate, HtmlTemplate},
};
use axum::{extract::State, response::IntoResponse, Extension};
use chrono::{DateTime, Local};
use tower_sessions::Session;

use crate::{db::create_number, middleware::FROM_PROTECTED_KEY, state::AppState};

pub async fn add_code(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, AddNumberError> {
    let current_local: DateTime<Local> = Local::now();
    let code = current_local.format("V%Y%m%d").to_string();

    // Create the new number
    let created_code = create_number(&state.db, &code, &user.id.to_string()).await?;

    Ok(HtmlTemplate(NumberTemplate { code: created_code }))
}

pub async fn change_password(session: Session) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    HtmlTemplate(PasswordChangeTemplate { from_protected })
}
