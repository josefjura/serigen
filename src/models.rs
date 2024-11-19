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

impl Into<Code> for CodeEntity {
    fn into(self) -> Code {
        Code {
            code: self.code,
            created_at: format_date(self.created_at),
            user_name: self.user_name,
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

impl Into<CodeValue> for CodeValueEntity {
    fn into(self) -> CodeValue {
        CodeValue { code: self.code }
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

impl Into<User> for UserEntity {
    fn into(self) -> User {
        User {
            id: self.id,
            name: self.name,
            is_admin: if self.is_admin == 1 { true } else { false },
        }
    }
}
