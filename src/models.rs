use chrono::NaiveDateTime;

use crate::utils::format_date;

pub struct CodeEntity {
    #[allow(dead_code)]
    pub id: i64,
    pub code: String,
    pub created_at: NaiveDateTime,
    #[allow(dead_code)]
    pub user_id: i64,
    pub user_name: String,
}

#[derive(Debug)]
pub struct Code {
    pub code: String,
    pub created_at: String,
    pub user_name: String,
}

impl From<CodeEntity> for Code {
    fn from(code: CodeEntity) -> Self {
        Code {
            code: code.code,
            created_at: format_date(code.created_at),
            user_name: code.user_name,
        }
    }
}

pub struct CodeValueEntity {
    pub code: String,
}

#[derive(Debug)]
pub struct CodeValue {
    pub code: String,
}

impl From<CodeValueEntity> for CodeValue {
    fn from(code: CodeValueEntity) -> Self {
        CodeValue { code: code.code }
    }
}

#[derive(Debug)]
pub struct UserEntity {
    #[allow(dead_code)]
    pub id: i64,
    pub name: String,
    pub password: String,
    pub is_admin: i64,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    #[allow(dead_code)]
    pub name: String,
    pub is_admin: bool,
}

impl From<UserEntity> for User {
    fn from(val: UserEntity) -> Self {
        User {
            id: val.id,
            name: val.name,
            is_admin: val.is_admin == 1,
        }
    }
}
