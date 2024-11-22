use sqlx::{
    error::ErrorKind,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tracing::info;

use crate::{
    errors::{
        add_number::AddNumberError, check_user_password::CheckUserPasswordError,
        create_user::CreateUserError, delete_user::DeleteUserError,
        password_change::ChangePasswordError, read_user::ReadUserError, read_users::ReadUsersError,
        reset_codes::ResetCodesError,
    },
    jwt::{hash_password, verify_password},
    models::{Code, CodeEntity, CodeValue, User, UserEntity},
};

pub async fn create_db_pool(path: &str) -> Result<SqlitePool, sqlx::Error> {
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

pub async fn read_last_ten(db: &SqlitePool) -> sqlx::Result<Vec<Code>> {
    let users = sqlx::query_as!(
        CodeEntity,
        r#"
				SELECT codes.id, code, created_at, users.id as user_id, users.name as user_name
				FROM codes
				JOIN users ON codes.user_id = users.id
				ORDER BY created_at DESC
				LIMIT 10
			"#,
    )
    .fetch_all(db)
    .await?;

    Ok(users.into_iter().map(|x| x.into()).collect())
}

pub async fn read_latest_today(
    db: &SqlitePool,
    code_prefix: &str,
) -> sqlx::Result<Option<CodeValue>> {
    let pattern = format!("{}%", code_prefix);
    let code = sqlx::query_as!(
        CodeValue,
        r#"
	SELECT code
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
				SELECT codes.id, code, created_at, users.id as user_id, users.name as user_name
				FROM codes
				JOIN users ON codes.user_id = users.id
				WHERE codes.id = ?
		"#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(code.map(|c| c.into()))
}

pub async fn reset_codes(db: &SqlitePool) -> sqlx::Result<(), ResetCodesError> {
    sqlx::query!(
        r#"
		DELETE FROM codes
	"#
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn create_code(
    db: &SqlitePool,
    code: &str,
    user_id: &str,
) -> sqlx::Result<Code, AddNumberError> {
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
    let new_code = format!("{}.{:0>2}", code, suffix + 1);
    let users = sqlx::query_scalar!(
        r#"
		INSERT INTO codes (code, user_id)
		VALUES (?, ?)
	"#,
        new_code,
        user_id
    )
    .execute(db)
    .await
    .map_err(|e: sqlx::Error| {
        if let Some(db_error) = e.as_database_error() {
            if db_error.kind() == ErrorKind::UniqueViolation {
                return AddNumberError::DuplicateCode(new_code.to_string());
            }
        }
        AddNumberError::DbError(e)
    })?;

    let code = read_code(db, users.last_insert_rowid()).await?.unwrap();

    Ok(code)
}

pub async fn read_user_by_id(db: &SqlitePool, user_id: &str) -> sqlx::Result<User, ReadUserError> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
					SELECT id, name, password, is_admin
					FROM users
					WHERE id = ?
				"#,
        user_id
    )
    .fetch_optional(db)
    .await?;

    if user.is_none() {
        return Err(ReadUserError::UserNotFound(user_id.to_string()));
    }

    Ok(user.unwrap().into())
}

pub async fn check_email_password(
    email: String,
    password: String,
    db: &SqlitePool,
) -> Result<User, CheckUserPasswordError> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
					SELECT id, name, password, is_admin
					FROM users
					WHERE name = ?
				"#,
        email
    )
    .fetch_optional(db)
    .await?;

    if user.is_none() {
        return Err(CheckUserPasswordError::NotValid);
    }

    let user = user.unwrap();

    if verify_password(&password, &user.password) {
        Ok(user.into())
    } else {
        Err(CheckUserPasswordError::NotValid)
    }
}

pub async fn change_password(
    db: &SqlitePool,
    user_id: i64,
    hashed_password: &str,
) -> sqlx::Result<(), ChangePasswordError> {
    sqlx::query!(
        r#"
				UPDATE users
				SET password = ?
				WHERE id = ?
			"#,
        hashed_password,
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| ChangePasswordError::DbError(e))?;

    Ok(())
}

pub async fn read_all_users(db: &SqlitePool) -> sqlx::Result<Vec<User>, ReadUsersError> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
				SELECT id, name, password, is_admin
				FROM users
			"#
    )
    .fetch_all(db)
    .await?;

    Ok(user.into_iter().map(|x| x.into()).collect())
}

pub async fn read_user(db: &SqlitePool, id: i64) -> sqlx::Result<Option<User>, ReadUsersError> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
				SELECT id, name, password, is_admin
				FROM users
				WHERE id = ?
			"#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(user.map(|x| x.into()))
}

pub async fn delete_user(db: &SqlitePool, id: i64) -> sqlx::Result<(), DeleteUserError> {
    let count = sqlx::query_scalar!(
        r#"
					SELECT COUNT(*) FROM users
					WHERE id <> ? and is_admin = 1
				"#,
        id
    )
    .fetch_one(db)
    .await?;

    if count == 0 {
        Err(DeleteUserError::CantDeleteLastAdmin)?;
    }

    sqlx::query_as!(
        UserEntity,
        r#"
					DELETE FROM users
					WHERE id = ?
				"#,
        id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn create_user(
    db: &SqlitePool,
    username: String,
    password: String,
    is_admin: bool,
) -> sqlx::Result<User, CreateUserError> {
    let hashed_password = hash_password(&password);

    let user = sqlx::query_as!(
        UserEntity,
        r#"
		INSERT INTO users (name, password, is_admin)
		VALUES (?, ?, ?)
	"#,
        username,
        hashed_password,
        is_admin
    )
    .execute(db)
    .await?;

    let user = read_user(db, user.last_insert_rowid()).await;

    match user {
        Ok(Some(user)) => Ok(user),
        _ => Err(CreateUserError::CantRead),
    }
}
