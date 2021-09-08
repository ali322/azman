use crate::repository::{DBError, POOL, Dao};
use app_macro::Dao;
use serde::Serialize;
use async_trait::async_trait;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "role_has_perms")]
#[derive(Debug, Clone, Dao)]
pub struct RolePermDao {
    pub role_id: i32,
    pub perm_id: i32,
}

impl RolePermDao{
  pub async fn find_by_id(role_id: i32, perm_id: i32) -> Result<Self, DBError> {
    let w = POOL.new_wrapper().eq("role_id", role_id).and().eq("perm_id", perm_id);
    Self::find_one(&w).await
  }
  pub async fn find_by_role(role_id: i32) -> Result<Vec<Self>, DBError> {
    let w = POOL.new_wrapper().eq("role_id", role_id);
    Self::find_list(&w).await
  }
}