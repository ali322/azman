use crate::repository::{Dao, dao::PermDao, vo::Perm, DBError, POOL};
use chrono::{Local, NaiveDateTime};
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
    pub created_by: Option<String>,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewPerm {
    pub async fn create(self) -> Result<Perm, DBError> {
        let mut dao = PermDao {
            id: None,
            name: self.name,
            description: self.description,
            value: self.value,
            domain_id: self.domain_id,
            is_deleted: Some(0),
            created_by: self.created_by.clone(),
            updated_by: self.created_by,
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
    pub updated_by: Option<String>,
}

impl UpdatePerm {
    pub async fn save(self, id: i32) -> Result<Perm, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = PermDao::find_one(&w).await?;
        dao.name = self.name;
        dao.description = self.description;
        dao.value = self.value;
        dao.updated_by = self.updated_by;
        PermDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}
