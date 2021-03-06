use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::repository::dao;
use super::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain{
  id: String,
  name: String,
  pub admin: Vec<User>,
  created_at: NaiveDateTime,
  updated_at: NaiveDateTime,
}

impl From<dao::Domain> for Domain{
  fn from(d: dao::Domain) -> Self {
      Self{
        id: d.id,
        name: d.name,
        admin: vec![],
        created_at: d.created_at,
        updated_at: d.updated_at
      }
  }
}