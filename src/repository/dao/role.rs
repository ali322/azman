use crate::{repository::{DBError, POOL, Dao}, util::serde_format::{naive_datetime, i32_bool}};
use app_macro::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "roles")]
#[derive(Debug, Clone, Dao)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
    pub level: i32,
    pub domain_id: String,
    #[serde(serialize_with = "i32_bool::serialize")]
    pub is_deleted: i32,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl Role{
  pub async fn find_by_ids(id: Vec<String>, domain_id: Option<String>) -> Result<Vec<Self>, DBError> {
    let mut w = POOL.new_wrapper().r#in("id", &id);
    if let Some(domain_id) = domain_id {
      w = w.eq("domain_id", domain_id);
    }
    Self::find_list(&w).await
  }
  pub async fn find_all(domain_id: Option<String>) -> Result<Vec<Self>, DBError> {
    let mut w = POOL.new_wrapper();
    if let Some(domain_id) = domain_id {
      w = w.eq("domain_id", domain_id);
    }
    Self::find_list(&w).await
  }
}