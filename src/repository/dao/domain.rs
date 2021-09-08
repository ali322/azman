use crate::{
    repository::{DBError, Dao, POOL},
    util::datetime_format::naive_datetime,
};
use app_macro::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};
use serde::Serialize;

#[crud_table(table_name: "domains")]
#[derive(Debug, Clone, Dao)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub default_role_id: Option<i32>,
    pub admin_role_id: Option<i32>,
    pub is_deleted: Option<i32>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
}
