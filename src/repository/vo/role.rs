use serde::{Serialize, Deserialize};
use crate::util::serde_format::{naive_datetime, i32_bool};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role{
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub domain_id: Option<String>,
  pub domain_name: Option<String>,
  pub value: String,
  pub level: i32,
  #[serde(serialize_with = "i32_bool::serialize")]
  pub is_deleted: i32,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub created_at: NaiveDateTime,
  #[serde(serialize_with = "naive_datetime::serialize")]
  pub updated_at: NaiveDateTime,
}