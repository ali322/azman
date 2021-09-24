use crate::{repository::{DBError, POOL, Dao}, util::serde_format::naive_datetime};
use app_macro::Dao;
use serde::Serialize;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "role_has_perms")]
#[derive(Debug, Clone, Dao)]
pub struct RolePerm {
    pub role_id: String,
    pub perm_id: String,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl RolePerm{
  pub async fn find_by_id(role_id: &str, perm_id: &str) -> Result<Self, DBError> {
    let w = POOL.new_wrapper().eq("role_id", role_id).and().eq("perm_id", perm_id);
    Self::find_one(w).await
  }
  pub async fn find_by_role(role_id: &str) -> Result<Vec<Self>, DBError> {
    let w = POOL.new_wrapper().eq("role_id", role_id);
    Self::find_list(w).await
  }
}