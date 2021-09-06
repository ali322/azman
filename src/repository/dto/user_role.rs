use crate::repository::{dao::UserRoleDao, vo::UserRole, DBError, POOL};
use app_macro::Dao;
use chrono::{Duration, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserGrantRole {
    pub user_id: String,
    pub role_id: i32
}

fn default_expire() -> NaiveDateTime {
    Local::now().naive_local() + Duration::days(30)
}

impl UserGrantRole {
    pub async fn save(self) -> Result<UserRole, DBError> {
        let dao = UserRoleDao {
            user_id: self.user_id,
            role_id: self.role_id,
            expire: default_expire(),
        };
        UserRoleDao::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserRole {
    pub expire: NaiveDateTime,
}

impl UpdateUserRole {
    pub async fn save(self, user_id: &str, role_id: i32) -> Result<UserRole, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", user_id)
            .and()
            .eq("role_id", role_id);
        let mut dao = UserRoleDao::find_one(&w).await?;
        dao.expire = self.expire;
        UserRoleDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserRevokeRole {
    pub user_id: String,
    pub role_id: i32,
}

impl UserRevokeRole {
    pub async fn save(self) -> Result<UserRole, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", self.user_id)
            .and()
            .eq("role_id", self.role_id);
        UserRoleDao::delete_one(&w).await?;
        UserRoleDao::find_one(&w).await.map(Into::into)
    }
}
