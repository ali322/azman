use crate::repository::{DBError, POOL};
use chrono::NaiveDateTime;
use app_macro::Dao;
use app_macro_derive::Dao;
use async_trait::async_trait;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "user_has_roles")]
#[derive(Debug, Clone, Dao)]
pub struct UserRoleDao {
    pub user_id: Option<String>,
    pub role_id: Option<i32>,
    pub expire: NaiveDateTime
}
