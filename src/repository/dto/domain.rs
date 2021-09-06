use crate::repository::{DBError, POOL, dao::{DomainDao, RoleDao}, vo::Domain};
use app_macro::Dao;
use chrono::{Local, NaiveDateTime};
use rbatis::crud::CRUDMut;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use std::env;

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
    pub async fn create(&self, user_id: String) -> Result<Domain, DBError> {
        let id = Uuid::new_v4().to_string();
        let admin_role_name =
            env::var("ADMIN_ROLE_NAME").expect("environment variable ADMIN_ROLE_NAME must be set");
        let common_role_name = env::var("COMMON_ROLE_NAME")
            .expect("environment variable COMMON_ROLE_NAME must be set");
        let mut tx = POOL.acquire_begin().await.unwrap();
        let new_role = RoleDao {
            id: None,
            name: admin_role_name.clone(),
            description: None,
            value: admin_role_name.clone(),
            level: 1,
            is_deleted: Some(0),
            domain_id: id.clone(),
            created_at: now(),
            updated_at: now(),
            created_by: Some(user_id.clone()),
            updated_by: Some(user_id.clone()),
        };
        let created = tx.save(&new_role, &[]).await?;
        let admin_role_id = created.last_insert_id.unwrap();
        let new_role = RoleDao {
          id: None,
          name: common_role_name.clone(),
          description: None,
          value: common_role_name.clone(),
          level: 999,
          is_deleted: Some(0),
          domain_id: id.clone(),
          created_at: now(),
          updated_at: now(),
          created_by: Some(user_id.clone()),
          updated_by: Some(user_id.clone()),
      };
      let created = tx.save(&new_role, &[]).await?;
      let common_role_id = created.last_insert_id.unwrap();
        let dao = DomainDao {
            id: id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            default_role_id: Some(common_role_id as i32),
            admin_role_id: Some(admin_role_id as i32),
            is_deleted: Some(0),
            created_at: now(),
            updated_at: now(),
        };
        tx.save(&dao, &[]).await?;
        // DomainDao::create_one(&dao).await?;
        tx.commit().await.unwrap();
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
    pub async fn save(&self, id: String) -> Result<Domain, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = DomainDao::find_one(&w).await?;
        dao.name = self.name.clone();
        dao.description = self.description.clone();
        DomainDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}
