use crate::{
    forms::{ChangePasswordSchema, LoginUserSchema},
    models::User,
    templates::{
        auth::{ChangePasswordSuccessTemplate, ChangePasswordTemplate, LoginTemplate},
        errors::Error500Template,
        HtmlTemplate,
    },
};
use axum::{
    extract::State,
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse, Redirect, Response},
    Extension, Form,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use tower_sessions::{
    cookie::{time::Duration, Cookie, SameSite},
    Session,
};
use tracing::{error, info};

use crate::{
    db::check_email_password, jwt::TokenClaims, middleware::FROM_PROTECTED_KEY, state::AppState,
};

pub async fn login(session: Session) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    HtmlTemplate(LoginTemplate {
        from_protected,
        is_admin: false,
        username: "".to_string(),
        password: "".to_string(),
        error: None,
    })
}

pub async fn login_post(
    State(state): State<AppState>,
    Form(form_data): Form<LoginUserSchema>,
) -> Result<Response, Response> {
    info!("Form: {:?}", form_data);

    let result = check_email_password(
        form_data.username.clone(),
        form_data.password.clone(),
        &state.db,
    )
    .await;

    if form_data.username.is_empty() || form_data.password.is_empty() {
        Err(HtmlTemplate(LoginTemplate {
            from_protected: false,
            is_admin: false,
            username: form_data.username.clone(),
            password: form_data.password,
            error: Some("Username or password cannot be empty".to_string()),
        })
        .into_response())?
    }

    if let Err(err) = result {
        let err = format!("Something went wrong: {}", err);
        error!("{}", err);
        return Err(HtmlTemplate(LoginTemplate {
            from_protected: false,
            is_admin: false,
            username: form_data.username,
            password: "".to_string(),
            error: Some("Wrong username or password".to_string()),
        })
        .into_response())?;
    }

    let user_id = result.unwrap().id;

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::days(7)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&state.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let headers = AppendHeaders([(SET_COOKIE, cookie.to_string())]);

    Ok((headers, Redirect::to("/")).into_response())
}

pub async fn logout_post(session: Session) -> impl IntoResponse {
    session.insert(FROM_PROTECTED_KEY, false).await.unwrap();

    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let headers = AppendHeaders([(SET_COOKIE, cookie.to_string())]);

    (headers, Redirect::to("/login"))
}

pub async fn change_password_post(
    session: Session,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Form(form): Form<ChangePasswordSchema>,
) -> Result<Response, Response> {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    // Check if the old password is correct
    let is_valid = check_email_password(user.name.clone(), form.old_password.clone(), &state.db)
        .await
        .map(|user| user.id == user.id)
        .unwrap_or(false);

    // Check if old and new passwords are the same
    if !is_valid {
        Err(HtmlTemplate(ChangePasswordTemplate {
            is_admin: user.is_admin,
            from_protected,
            error: Some("Old password is incorrect".to_string()),
        })
        .into_response())?
    }

    // Check if old and new passwords are the same
    if form.old_password == form.new_password {
        Err(HtmlTemplate(ChangePasswordTemplate {
            is_admin: user.is_admin,
            from_protected,
            error: Some("Old and new password cannot be the same".to_string()),
        })
        .into_response())?
    }

    // Check if the new password and retype password are the same
    if form.new_password != form.retype_password {
        Err(HtmlTemplate(ChangePasswordTemplate {
            is_admin: user.is_admin,
            from_protected,
            error: Some("New password and retype password do not match".to_string()),
        })
        .into_response())?
    }

    // Hash the new password
    let hashed_password = crate::jwt::hash_password(&form.new_password);

    // Update the password
    let result = crate::db::change_password(&state.db, user.id, &hashed_password).await;

    if let Err(err) = result {
        let err = format!("Something went wrong: {}", err);
        error!("{}", err);
        Err(HtmlTemplate(Error500Template {
            is_admin: user.is_admin,
            from_protected,
            reason: "Failed to change password".to_string(),
        })
        .into_response())?
    }

    Ok(HtmlTemplate(ChangePasswordSuccessTemplate {
        is_admin: user.is_admin,
        from_protected,
    })
    .into_response())
}
