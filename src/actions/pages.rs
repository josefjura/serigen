use crate::{
    db::read_filtered,
    models::{Code, User},
    templates::{
        pages::{IndexPageTemplate, ListPageTemplate, ListSectionTemplate},
        HtmlTemplate,
    },
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{db::read_last_ten, middleware::FROM_PROTECTED_KEY, state::AppState};

pub async fn index(
    session: Session,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    let last_ten = read_last_ten(&state.db).await;

    if let Ok(last_ten) = last_ten {
        Ok(HtmlTemplate(IndexPageTemplate {
            codes: last_ten,
            from_protected,
            is_admin: user.is_admin,
            logged_user: Some(user.name.clone()),
        }))
    } else {
        Err("Failed to read last ten numbers")
    }
}

#[derive(Deserialize)]
pub struct ListPageQuery {
    page: Option<u32>,
    per_page: Option<u32>,
    code: Option<String>,
    user_name: Option<String>,
}

struct ListResults {
    codes: Vec<Code>,
    total: i64,
    page: u32,
    per_page: u32,
    code_filter: String,
    user_filter: String,
    total_pages: u32,
    has_prev: bool,
    has_next: bool,
    prev_page: u32,
    next_page: u32,
}

async fn fetch_list_results(
    db: &sqlx::SqlitePool,
    query: &ListPageQuery,
) -> sqlx::Result<ListResults> {
    let page = query.page.unwrap_or(0);
    let per_page = query.per_page.unwrap_or(15);
    let code_filter = query.code.clone().unwrap_or_default();
    let user_filter = query.user_name.clone().unwrap_or_default();

    let results = read_filtered(
        db,
        per_page,
        per_page * page,
        &code_filter,
        &user_filter,
    )
    .await?;

    let total_pages = if per_page == 0 {
        0
    } else {
        (results.total.max(0) as u32).div_ceil(per_page)
    };
    let has_prev = page > 0;
    let has_next = page + 1 < total_pages;
    let prev_page = page.saturating_sub(1);
    let next_page = if has_next { page + 1 } else { page };

    Ok(ListResults {
        codes: results.codes,
        total: results.total,
        page,
        per_page,
        code_filter,
        user_filter,
        total_pages,
        has_prev,
        has_next,
        prev_page,
        next_page,
    })
}

pub async fn list(
    session: Session,
    query: Query<ListPageQuery>,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    if let Ok(results) = fetch_list_results(&state.db, &query).await {
        Ok(HtmlTemplate(ListPageTemplate {
            codes: results.codes,
            total: results.total,
            page: results.page,
            per_page: results.per_page,
            code_filter: results.code_filter,
            user_filter: results.user_filter,
            total_pages: results.total_pages,
            has_prev: results.has_prev,
            has_next: results.has_next,
            prev_page: results.prev_page,
            next_page: results.next_page,
            from_protected,
            is_admin: user.is_admin,
            logged_user: Some(user.name.clone()),
        }))
    } else {
        Err("Failed to read filtered numbers")
    }
}

pub async fn list_section(
    query: Query<ListPageQuery>,
    State(state): State<AppState>
) -> impl IntoResponse {
    if let Ok(results) = fetch_list_results(&state.db, &query).await {
        Ok(HtmlTemplate(ListSectionTemplate {
            codes: results.codes,
            total: results.total,
            page: results.page,
            per_page: results.per_page,
            code_filter: results.code_filter,
            user_filter: results.user_filter,
            total_pages: results.total_pages,
            has_prev: results.has_prev,
            has_next: results.has_next,
            prev_page: results.prev_page,
            next_page: results.next_page,
        }))
    } else {
        Err("Failed to read filtered numbers")
    }
}
