use crate::repository::{dao::PermDao, vo::Role, DBError, POOL};
use chrono::{Local, NaiveDateTime};
use rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewPerm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
    #[serde(skip_deserializing)]
    pub domain_id: String,
    #[serde(skip_deserializing)]
    pub created_by: String,
    #[serde(skip_deserializing)]
    pub updated_by: String,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewPerm {
    pub fn copy_with(self, domain_id: String, user_id: String) -> Self {
        Self {
            domain_id: domain_id.clone(),
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            ..self
        }
    }
    pub async fn create(&self) -> Result<Role, DBError> {
        let dao = PermDao {
            id: None,
            name: self.name.clone(),
            description: self.description.clone(),
            value: self.value.clone(),
            domain_id: self.domain_id.clone(),
            created_by: self.created_by.clone(),
            updated_by: self.updated_by.clone(),
            created_at: now(),
            updated_at: now(),
        };
        let ret = POOL.save(&dao, &[]).await?;
        Role::find_one(ret.last_insert_id.unwrap() as i32).await
    }
}
