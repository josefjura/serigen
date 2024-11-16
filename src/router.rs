use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::actions::{add_number, index};

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/number", post(add_number))
        .nest_service("/assets", ServeDir::new("assets"))
}
