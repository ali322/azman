use crate::repository::{DBError, POOL};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};
use app_macro::Dao;
use app_macro_derive::Dao;
use async_trait::async_trait;

#[crud_table(table_name: "perms")]
#[derive(Debug, Clone, Dao)]
pub struct PermDao {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
    pub domain_id: String,
    pub is_deleted: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}
