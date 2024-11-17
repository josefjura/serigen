use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::{
    actions::{add_number, index, login, login_post, logout_post},
    context::AppContext,
    middleware::auth_middleware,
};

pub fn setup_router(db: SqlitePool, jwt_secret: &str) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);
    let app_state = AppContext::new(db, jwt_secret);
    Router::new()
        .route(
            "/",
            get(index).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/number",
            post(add_number).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
        )
        .route("/login", get(login).post(login_post))
        .route("/logout", post(logout_post))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
