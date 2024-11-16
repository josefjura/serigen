use chrono::NaiveDateTime;
use sqlx::{
    error::{self, ErrorKind},
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tracing::info;

use crate::errors::AddNumberError;

pub async fn setup_db(path: &str) -> Result<SqlitePool, sqlx::Error> {
    info!("Setting up database at {}", path);
    let opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    let db = SqlitePoolOptions::new().connect_with(opts).await?;
    info!("Connected to database");
    sqlx::migrate!().run(&db).await?;
    info!("Migrated database");
    Ok(db)
}

pub struct CodeEntity {
    #[allow(dead_code)]
    pub id: i64,
    pub code: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Code {
    pub code: String,
    pub created_at: String,
}

impl Into<Code> for CodeEntity {
    fn into(self) -> Code {
        Code {
            code: self.code,
            created_at: format_date(self.created_at),
        }
    }
}

fn format_date(date: NaiveDateTime) -> String {
    if date.date() == chrono::Local::now().date_naive() {
        return date.format("%H:%M:%S").to_string();
    }
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub async fn read_last_ten(db: &SqlitePool) -> sqlx::Result<Vec<Code>> {
    let users = sqlx::query_as!(
        CodeEntity,
        r#"
				SELECT id, code, created_at
				FROM codes
				ORDER BY created_at DESC
				LIMIT 10
			"#,
    )
    .fetch_all(db)
    .await?;

    Ok(users.into_iter().map(|x| x.into()).collect())
}

pub async fn read_latest_today(db: &SqlitePool, code_prefix: &str) -> sqlx::Result<Option<Code>> {
    let pattern = format!("{}%", code_prefix);
    let code = sqlx::query_as!(
        CodeEntity,
        r#"
	SELECT id, code, created_at
	FROM codes
	WHERE code LIKE ?
	ORDER By created_at DESC
	LIMIT 1
"#,
        pattern
    )
    .fetch_optional(db)
    .await?;

    Ok(code.map(|c| c.into()))
}

pub async fn read_code(db: &SqlitePool, id: i64) -> sqlx::Result<Option<Code>> {
    let code = sqlx::query_as!(
        CodeEntity,
        r#"
		SELECT id, code, created_at
		FROM codes
		WHERE id = ?
	"#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(code.map(|c| c.into()))
}

pub async fn create_number(db: &SqlitePool, code: &str) -> sqlx::Result<Code, AddNumberError> {
    info!("Creating new code with prefix: {}", code);
    let latest_code = read_latest_today(db, &code).await?;
    let suffix = match latest_code {
        Some(code) => code
            .code
            .split('.')
            .last()
            .ok_or_else(|| AddNumberError::ParseSuffixError(code.code.clone()))?
            .parse::<i64>()
            .map_err(|_| AddNumberError::ParseSuffixError(code.code.clone()))?,
        None => 0, // No existing code, start at 0
    };

    // Generate the new code
    let new_code = format!("{}.{}", code, suffix + 1);
    let users = sqlx::query_scalar!(
        r#"
		INSERT INTO codes (code)
		VALUES (?)
	"#,
        new_code
    )
    .execute(db)
    .await
    .map_err(|e: sqlx::Error| {
        if let Some(db_error) = e.as_database_error() {
            if db_error.kind() == ErrorKind::UniqueViolation {
                return AddNumberError::DuplicateCode(new_code.to_string());
            }
        }
        AddNumberError::UnknownError(e)
    })?;

    let code = read_code(db, users.last_insert_rowid()).await?.unwrap();

    Ok(code)
}
