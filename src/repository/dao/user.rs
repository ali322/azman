use crate::repository::{DBError, POOL};
use app_macro::Dao;
use app_macro_trait::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};
use serde::Serialize;

#[crud_table(table_name: "users")]
#[derive(Debug, Clone, Dao)]
pub struct UserDao {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
    pub sys_role: Option<String>,
    pub is_actived: Option<i32>,
    pub last_logined_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl UserDao {
    pub async fn find_by_username(username: &str) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("username", username);
        Self::find_one(&w).await
    }
    pub async fn find_by_username_or_email(username_or_email: &str) -> Result<Self, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("username", username_or_email)
            .or()
            .eq("email", username_or_email);
        Self::find_one(&w).await
    }
}
