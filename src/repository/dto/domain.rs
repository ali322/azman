use crate::{
    repository::{
        dao::{Domain, Role},
        DBError, Dao, POOL,
    },
    util::{now, uuid_v4},
};
use rbatis::crud::CRUDMut;
use serde::{Deserialize, Serialize};
use std::env;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewDomain {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub default_role_id: Option<i32>,
    pub admin_role_id: Option<i32>,
}

impl NewDomain {
    pub async fn create(self, user_id: &str) -> Result<Domain, DBError> {
        let domain_id = uuid_v4();
        let admin_role_name =
            env::var("ADMIN_ROLE_NAME").expect("environment variable ADMIN_ROLE_NAME must be set");
        let common_role_name = env::var("COMMON_ROLE_NAME")
            .expect("environment variable COMMON_ROLE_NAME must be set");
        let mut tx = POOL.acquire_begin().await.unwrap();
        let admin_role_id = uuid_v4();
        let new_role = Role {
            id: admin_role_id.clone(),
            name: admin_role_name.clone(),
            description: None,
            value: admin_role_name.clone(),
            level: 1,
            is_deleted: 0,
            domain_id: domain_id.clone(),
            created_at: now(),
            updated_at: now(),
            created_by: Some(user_id.to_string()),
            updated_by: Some(user_id.to_string()),
        };
        tx.save(&new_role, &[]).await?;
        let common_role_id = uuid_v4();
        let new_role = Role {
            id: common_role_id.clone(),
            name: common_role_name.clone(),
            description: None,
            value: common_role_name.clone(),
            level: 999,
            is_deleted: 0,
            domain_id: domain_id.clone(),
            created_at: now(),
            updated_at: now(),
            created_by: Some(user_id.to_string()),
            updated_by: Some(user_id.to_string()),
        };
        tx.save(&new_role, &[]).await?;
        let dao = Domain {
            id: domain_id.clone(),
            name: self.name,
            description: self.description,
            default_role_id: Some(common_role_id),
            admin_role_id: Some(admin_role_id),
            is_deleted: 0,
            created_at: now(),
            updated_at: now(),
        };
        tx.save(&dao, &[]).await?;
        // DomainDao::create_one(&dao).await?;
        tx.commit().await.unwrap();
        Ok(dao)
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateDomain {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
}

impl UpdateDomain {
    pub async fn save(self, id: &str) -> Result<Domain, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = Domain::find_one(&w).await?;
        dao.name = self.name;
        dao.description = self.description;
        Domain::update_one(&dao, &w).await?;
        Ok(dao)
    }
}
