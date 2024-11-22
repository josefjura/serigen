use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tower_sessions::Session;

use crate::{
    db::read_user_by_id,
    jwt::TokenClaims,
    state::AppState,
    templates::{errors::Error401Template, HtmlTemplate},
};

pub const FROM_PROTECTED_KEY: &str = "from_protected";

pub async fn auth_middleware(
    State(state): State<AppState>,
    session: Session,
    cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let token_option = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.strip_prefix("Bearer ").map(|s| s.to_string()))
        });

    let token = if let Some(tk) = token_option {
        tk
    } else {
        session.insert(FROM_PROTECTED_KEY, false).await.unwrap();

        Err(Redirect::to("/login").into_response())?
    };

    let claims = if let Ok(clm) = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        clm.claims
    } else {
        Err(HtmlTemplate(Error401Template {
            reason: "Invalid token".to_string(),
            from_protected: false,
            is_admin: false,
        })
        .into_response())?
    };

    let user_id = &claims.sub;
    let user = read_user_by_id(&state.db, user_id).await;

    match user {
        Ok(user) => {
            session.insert(FROM_PROTECTED_KEY, true).await.unwrap();

            req.extensions_mut().insert(user);
        }
        Err(e) => Err(HtmlTemplate(Error401Template {
            reason: e.to_string(),
            from_protected: false,
            is_admin: false,
        })
        .into_response())?,
    }

    Ok::<Response, _>(next.run(req).await)
}
