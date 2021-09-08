use crate::repository::{DBError, POOL, Dao};
use app_macro::Dao;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "domains")]
#[derive(Debug, Clone, Dao)]
pub struct DomainDao {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub default_role_id: Option<i32>,
    pub admin_role_id: Option<i32>,
    pub is_deleted: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
