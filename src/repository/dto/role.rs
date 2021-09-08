use crate::{repository::{Dao, dao::Role, DBError, POOL}, util::{now, uuid_v4}};
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

impl NewRole {
    pub async fn create(self) -> Result<Role, DBError> {
        let id = uuid_v4();
        let dao = Role {
            id,
            name: self.name,
            description: self.description,
            value: self.value,
            level: self.level,
            domain_id: self.domain_id,
            is_deleted: 0,
            created_by: self.created_by.clone(),
            updated_by: self.created_by,
            created_at: now(),
            updated_at: now(),
        };
        Role::create_one(&dao).await?;
        Ok(dao)
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
    pub async fn save(self, id: &str) -> Result<Role, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = Role::find_one(&w).await?;
        dao.name = self.name;
        dao.description = self.description;
        dao.value = self.value;
        dao.level = self.level;
        dao.updated_by = self.updated_by;
        Role::update_one(&dao, &w).await?;
        Ok(dao)
    }
}
