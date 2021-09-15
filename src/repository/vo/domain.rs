use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::repository::dao;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain{
  id: String,
  name: String,
  created_at: NaiveDateTime,
  updated_at: NaiveDateTime,
}

impl From<dao::Domain> for Domain{
  fn from(d: dao::Domain) -> Self {
      Self{
        id: d.id,
        name: d.name,
        created_at: d.created_at,
        updated_at: d.updated_at
      }
  }
}