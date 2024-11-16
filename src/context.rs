use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: SqlitePool,
}

impl AppContext {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}
