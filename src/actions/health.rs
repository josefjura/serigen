use axum::{extract::State, http::StatusCode, response::IntoResponse};
use tracing::warn;

use crate::state::AppState;

/// Liveness/readiness probe. Returns 200 `ok` when the database answers `SELECT 1`,
/// 503 otherwise. Mounted outside the session, auth and trace layers.
pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(e) => {
            warn!("Health check failed: {}", e);
            (StatusCode::SERVICE_UNAVAILABLE, "db unavailable")
        }
    }
}
