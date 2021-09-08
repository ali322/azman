use crate::{
    repository::{Dao, dao::UserRoleDao, vo::UserRole, DBError, POOL},
    util::datetime_format::naive_datetime,
};
use chrono::{Duration, Local, NaiveDateTime};
use rbatis::crud::CRUDMut;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserGrantRole {
    pub user_id: String,
    pub role_id: i32,
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
    pub user_id: String,
    pub role_id: i32,
    #[serde(with = "naive_datetime")]
    pub expire: NaiveDateTime,
}

impl UpdateUserRole {
    pub async fn save(self) -> Result<UserRole, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", self.user_id)
            .and()
            .eq("role_id", self.role_id);
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
    pub async fn save(self) -> Result<u64, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", self.user_id)
            .and()
            .eq("role_id", self.role_id);
        UserRoleDao::delete_one(&w).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChangeRole {
    pub user_id: String,
    pub role_ids: Vec<i32>,
}

impl UserChangeRole {
    pub async fn save(self) -> Result<Vec<UserRole>, DBError> {
        let mut tx = POOL.acquire_begin().await.unwrap();
        let w = POOL.new_wrapper().eq("user_id", &self.user_id);
        tx.remove_by_wrapper::<UserRoleDao>(&w).await?;
        let rows: Vec<UserRoleDao> = self
            .role_ids
            .iter()
            .map(|role_id| UserRoleDao {
                user_id: self.user_id.clone(),
                role_id: *role_id,
                expire: default_expire(),
            })
            .collect();
        tx.save_batch(&rows, &[]).await?;
        tx.commit().await.unwrap();
        let all: Vec<UserRole> = rows.into_iter().map(|v| v.into()).collect();
        Ok(all)
    }
}
