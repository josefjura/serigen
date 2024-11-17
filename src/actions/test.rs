use sqlx::SqlitePool;

use crate::errors::AddNumberError;

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
