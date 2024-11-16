use axum::{response::IntoResponse, Extension};
use chrono::{DateTime, Local};

#[cfg(test)]
mod test;

use crate::{
    context::AppContext,
    db::{create_number, read_last_ten, read_latest_today},
    errors::AddNumberError,
    templates::{HtmlTemplate, IndexTemplate, NumberTemplate},
};

pub async fn index(ctx: Extension<AppContext>) -> impl IntoResponse {
    let last_ten = read_last_ten(&ctx.db).await;

    if let Ok(last_ten) = last_ten {
        Ok(HtmlTemplate(IndexTemplate { codes: last_ten }))
    } else {
        Err("Failed to read last ten numbers")
    }
}

pub async fn add_number(ctx: Extension<AppContext>) -> Result<impl IntoResponse, AddNumberError> {
    let current_local: DateTime<Local> = Local::now();
    let code = current_local.format("V%Y%m%d").to_string();

    // Create the new number
    let created_code = create_number(&ctx.db, &code).await?;

    Ok(HtmlTemplate(NumberTemplate { code: created_code }))
}
