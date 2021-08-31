use crate::{
  repository::{dao::PermDao, DBError, POOL},
  util::datetime_format::naive_datetime,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perm {
  pub id: Option<i32>,
  pub name: String,
  pub description: Option<String>,
  pub value: String,
  pub domain_id: String,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub created_at: NaiveDateTime,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub updated_at: NaiveDateTime,
  pub created_by: String,
  pub updated_by: String,
}

impl From<PermDao> for Perm {
  fn from(dao: PermDao) -> Self {
      Self {
          id: dao.id,
          name: dao.name,
          description: dao.description,
          value: dao.value,
          domain_id: dao.domain_id,
          created_by: dao.created_by,
          updated_by: dao.updated_by,
          created_at: dao.created_at,
          updated_at: dao.updated_at,
      }
  }
}

impl Perm {
  pub async fn find_one(id: i32) -> Result<Self, DBError> {
      let w = POOL.new_wrapper().eq("id", id);
      PermDao::find_one(&w).await.map(Into::into)
  }
  pub async fn find_all() -> Result<Vec<Self>, DBError> {
      let w = POOL.new_wrapper();
      let all = PermDao::find_list(&w).await?;
      let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
      Ok(all)
  }
}
