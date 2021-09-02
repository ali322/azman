use crate::repository::{DBError, POOL};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};
use app_macro::Dao;
use app_macro_derive::Dao;
use async_trait::async_trait;

#[crud_table(table_name: "orgs")]
#[derive(Debug, Clone, Dao)]
pub struct OrgDao {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub domain_id: String,
    pub is_deleted: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
