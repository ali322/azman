use crate::repository::{dao::DomainDao, vo::Domain, DBError, POOL};
use app_macro::Dao;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewDomain {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub default_role_id: Option<i32>,
    pub admin_role_id: Option<i32>,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewDomain {
    pub async fn create(&self) -> Result<Domain, DBError> {
        let mut dao = DomainDao {
            id: None,
            name: self.name.clone(),
            description: self.description.clone(),
            default_role_id: self.default_role_id,
            admin_role_id: self.admin_role_id,
            is_deleted: Some(0),
            created_at: now(),
            updated_at: now(),
        };
        let id = DomainDao::create_one(&dao).await?;
        dao.id = Some(id as i32);
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateDomain {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
}

impl UpdateDomain {
    pub async fn save(&self, id: i32) -> Result<Domain, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = DomainDao::find_one(&w).await?;
        dao.name = self.name.clone();
        dao.description = self.description.clone();
        DomainDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}
