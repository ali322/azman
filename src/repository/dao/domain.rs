use crate::{
    repository::{DBError, Dao, POOL, vo},
    util::serde_format::{naive_datetime, i32_bool},
};
use app_macro::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};
use serde::Serialize;
use super::{UserRole, User};

#[crud_table(table_name: "domains")]
#[derive(Debug, Clone, Dao)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub default_role_id: String,
    pub admin_role_id: String,
    #[serde(serialize_with = "i32_bool::serialize")]
    pub is_deleted: i32,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
}

impl Domain{
  pub async fn find_by_admin_role(role_ids: Vec<String>) -> Result<Vec<Self>, DBError>{
    let w = POOL.new_wrapper().r#in("admin_role_id", &role_ids);
    Self::find_list(w).await
  }
}

#[async_trait]
pub trait IntoVo{
  async fn into_vo(&self) -> Result<vo::Domain, DBError>;
}

#[async_trait]
impl IntoVo for Domain{
  async fn into_vo(&self) -> Result<vo::Domain, DBError>{
    let mut domain = vo::Domain::from(self.clone());
    let user_roles = UserRole::find_by_role(&self.admin_role_id).await?;
    let user_ids: Vec<String> = user_roles.into_iter().map(|v| v.user_id).collect();
    let users = if user_ids.len() > 0 {
        User::find_by_ids(user_ids).await?
    } else {
        vec![]
    };
    domain.admin = users.into_iter().map(Into::into).collect();
    Ok(domain)
  }
}