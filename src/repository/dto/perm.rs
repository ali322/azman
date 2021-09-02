use crate::repository::{dao::PermDao, vo::Perm, DBError, POOL};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;
use app_macro::Dao;

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
    pub created_by: Option<String>,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewPerm {
    pub async fn create(&self) -> Result<Perm, DBError> {
        let mut dao = PermDao {
            id: None,
            name: self.name.clone(),
            description: self.description.clone(),
            value: self.value.clone(),
            domain_id: self.domain_id.clone(),
            is_deleted: Some(0),
            created_by: self.created_by.clone(),
            updated_by: None,
            created_at: now(),
            updated_at: now(),
        };
        let id = PermDao::create_one(&dao).await?;
        dao.id = Some(id as i32);
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdatePerm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
    #[serde(skip_deserializing)]
    pub created_by: Option<String>,
}

impl UpdatePerm {
    pub async fn save(&self, id: i32) -> Result<Perm, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = PermDao::find_one(&w).await?;
        dao.name = self.name.clone();
        dao.description = self.description.clone();
        dao.value = self.value.clone();
        PermDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}
