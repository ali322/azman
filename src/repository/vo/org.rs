use serde::{Serialize, Deserialize};
use crate::{util::serde_format::{naive_datetime, i32_bool}, repository::dao};
use chrono::NaiveDateTime;
use super::Domain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Org{
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub domain_id: String,
  pub domain: Option<Domain>,
  #[serde(serialize_with = "i32_bool::serialize")]
  pub is_deleted: i32,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub created_at: NaiveDateTime,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub updated_at: NaiveDateTime,
}

impl From<dao::Org> for Org{
  fn from(d: dao::Org) -> Self {
      Self{
        id: d.id,
        name: d.name,
        description: d.description,
        domain_id: d.domain_id,
        domain: None,
        is_deleted: d.is_deleted,
        created_at: d.created_at,
        updated_at: d.updated_at
      }
  }
}