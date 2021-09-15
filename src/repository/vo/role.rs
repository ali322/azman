use serde::{Serialize, Deserialize};
use crate::{util::serde_format::{naive_datetime, i32_bool}, repository::dao};
use chrono::NaiveDateTime;
use super::Domain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role{
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub domain_id: String,
  pub domain: Option<Domain>,
  pub value: String,
  pub level: i32,
  #[serde(serialize_with = "i32_bool::serialize")]
  pub is_deleted: i32,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub created_at: NaiveDateTime,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub updated_at: NaiveDateTime,
}

impl From<dao::Role> for Role{
  fn from(d: dao::Role) -> Self {
      Self{
        id: d.id,
        name: d.name,
        description: d.description,
        domain_id: d.domain_id,
        domain: None,
        value: d.value,
        level: d.level,
        is_deleted: d.is_deleted,
        created_at: d.created_at,
        updated_at: d.updated_at
      }
  }
}