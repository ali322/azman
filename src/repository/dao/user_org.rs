use crate::repository::{DBError, POOL, Dao};
use app_macro::Dao;
use async_trait::async_trait;
use serde::Serialize;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "user_has_orgs")]
#[derive(Debug, Clone, Dao)]
pub struct UserOrgDao {
    pub user_id: String,
    pub org_id: String,
    pub expire: NaiveDateTime,
}

impl UserOrgDao{
  pub async fn find_by_id(user_id: &str, org_id: &str) -> Result<Self, DBError> {
    let w = POOL.new_wrapper().eq("user_id", user_id).and().eq("org_id", org_id);
    Self::find_one(&w).await
  }
  pub async fn find_by_user(user_id: &str) -> Result<Vec<Self>, DBError> {
    let w = POOL.new_wrapper().eq("user_id", user_id);
    Self::find_list(&w).await
  }
  pub async fn find_by_org(org_id: &str) -> Result<Vec<Self>, DBError> {
    let w = POOL.new_wrapper().eq("org_id", org_id);
    Self::find_list(&w).await
  }
}