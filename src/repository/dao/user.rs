use crate::repository::{DBError, POOL};
use app_macro::Dao;
use app_macro_derive::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

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
