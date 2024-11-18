use sqlx::SqlitePool;

use crate::{
    errors::{add_number::AddNumberError, check_user_password::CheckUserPasswordError},
    jwt::hash_password,
};

#[sqlx::test(fixtures("codes"))]
async fn read_last_ten(db: SqlitePool) -> sqlx::Result<()> {
    let codes = crate::db::read_last_ten(&db).await?;

    assert_eq!(codes.len(), 10);

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn read_code(db: SqlitePool) -> sqlx::Result<()> {
    let code = crate::db::read_code(&db, 1).await?;

    assert!(code.is_some());

    assert_eq!(code.unwrap().code, "V20240101.1");

    let code = crate::db::read_code(&db, 2).await?;

    assert!(code.is_some());

    assert_eq!(code.unwrap().code, "V20240101.2");

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn read_code_neg(db: SqlitePool) -> sqlx::Result<()> {
    let code = crate::db::read_code(&db, 69).await?;

    assert!(code.is_none());

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn read_latest_today(db: SqlitePool) -> sqlx::Result<()> {
    let code = crate::db::read_latest_today(&db, "V20240101").await?;

    assert!(code.is_some());

    assert_eq!(code.unwrap().code, "V20240101.3");

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn read_latest_today_with_spaces(db: SqlitePool) -> sqlx::Result<()> {
    let code = crate::db::read_latest_today(&db, "V20240106").await?;

    assert!(code.is_some());

    assert_eq!(code.unwrap().code, "V20240106.7");

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn read_latest_today_neg(db: SqlitePool) -> sqlx::Result<()> {
    let code = crate::db::read_latest_today(&db, "V20240107").await?;

    assert!(code.is_none());

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn create_number(db: SqlitePool) -> sqlx::Result<(), AddNumberError> {
    let code = crate::db::create_number(&db, "V20240106", "1").await?;

    assert_eq!(code.code, "V20240106.8");
    assert_eq!(code.user_name, "Admin");

    let code = crate::db::create_number(&db, "V20240106", "1").await?;

    assert_eq!(code.code, "V20240106.9");
    assert_eq!(code.user_name, "Admin");

    let code = crate::db::create_number(&db, "V20240106", "1").await?;

    assert_eq!(code.code, "V20240106.10");
    assert_eq!(code.user_name, "Admin");

    let code = crate::db::create_number(&db, "V20240106", "1").await?;

    assert_eq!(code.code, "V20240106.11");
    assert_eq!(code.user_name, "Admin");

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn read_user_by_id(db: SqlitePool) -> sqlx::Result<()> {
    let user = crate::db::read_user_by_id(&db, "1").await;

    assert!(user.is_ok());

    let user = user.unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Admin");

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn read_user_by_id_neg(db: SqlitePool) -> sqlx::Result<()> {
    let user = crate::db::read_user_by_id(&db, "69").await;

    assert!(user.is_err());

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn check_email_password(db: SqlitePool) -> sqlx::Result<()> {
    let hashed = hash_password("pass");
    let user = crate::db::check_email_password("Admin".to_string(), hashed.to_string(), &db).await;

    assert!(user.is_ok());

    let user = user.unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Admin");

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn check_email_password_neg(db: SqlitePool) -> sqlx::Result<()> {
    let hashed = hash_password("pass1");
    let user = crate::db::check_email_password("Admin".to_string(), hashed.to_string(), &db).await;

    match user {
        Err(CheckUserPasswordError::NotValid) => assert!(true),
        _ => assert!(false),
    }

    Ok(())
}

#[sqlx::test(fixtures("codes"))]
async fn change_password(db: SqlitePool) -> sqlx::Result<()> {
    let user = crate::db::change_password(&db, 1, "new_pass").await;

    assert!(user.is_ok());

    Ok(())
}
