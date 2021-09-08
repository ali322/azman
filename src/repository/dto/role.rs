use crate::repository::{Dao, dao::RoleDao, vo::Role, DBError, POOL};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewRole {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
    #[validate(range(min = 2, max = 99))]
    pub level: i32,
    #[serde(skip_deserializing)]
    pub domain_id: String,
    #[serde(skip_deserializing)]
    pub created_by: Option<String>
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewRole {
    pub async fn create(self) -> Result<Role, DBError> {
        let mut dao = RoleDao {
            id: None,
            name: self.name,
            description: self.description,
            value: self.value,
            level: self.level,
            domain_id: self.domain_id,
            is_deleted: Some(0),
            created_by: self.created_by.clone(),
            updated_by: self.created_by,
            created_at: now(),
            updated_at: now(),
        };
        let id = RoleDao::create_one(&dao).await?;
        dao.id = Some(id as i32);
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateRole {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
    #[validate(range(min = 2, max = 99))]
    pub level: i32,
    #[serde(skip_deserializing)]
    pub updated_by: Option<String>,
}

impl UpdateRole {
    pub async fn save(self, id: i32) -> Result<Role, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = RoleDao::find_one(&w).await?;
        dao.name = self.name;
        dao.description = self.description;
        dao.value = self.value;
        dao.level = self.level;
        dao.updated_by = self.updated_by;
        RoleDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}
