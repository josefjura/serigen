use axum::{
    extract::State,
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse, Redirect},
    Extension, Form,
};
use chrono::{DateTime, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use tower_sessions::{
    cookie::{time::Duration, Cookie, SameSite},
    Session,
};
use tracing::error;

#[cfg(test)]
mod test;

use crate::{
    context::AppContext,
    db::{check_email_password, create_number, read_last_ten, User},
    errors::AddNumberError,
    jwt::TokenClaims,
    middleware::FROM_PROTECTED_KEY,
    templates::{HtmlTemplate, IndexTemplate, LoginTemplate, LoginUserSchema, NumberTemplate},
};

pub async fn index(session: Session, State(state): State<AppContext>) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    let last_ten = read_last_ten(&state.db).await;

    if let Ok(last_ten) = last_ten {
        Ok(HtmlTemplate(IndexTemplate {
            codes: last_ten,
            from_protected,
        }))
    } else {
        Err("Failed to read last ten numbers")
    }
}

pub async fn login(session: Session) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    HtmlTemplate(LoginTemplate { from_protected })
}

pub async fn login_post(
    State(state): State<AppContext>,
    Form(form_data): Form<LoginUserSchema>,
) -> impl IntoResponse {
    let result = check_email_password(form_data.username, form_data.password, &state.db).await;

    if let Err(err) = result {
        let err = format!("Something went wrong: {}", err);
        error!("{}", err);
        return Redirect::to("/login").into_response();
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

    (headers, Redirect::to("/")).into_response()
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

pub async fn add_number(
    State(state): State<AppContext>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, AddNumberError> {
    let current_local: DateTime<Local> = Local::now();
    let code = current_local.format("V%Y%m%d").to_string();

    // Create the new number
    let created_code = create_number(&state.db, &code, &user.id.to_string()).await?;

    Ok(HtmlTemplate(NumberTemplate { code: created_code }))
}
