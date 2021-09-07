use crate::{
    repository::{dao::UserRoleDao, DBError, POOL},
    util::datetime_format::naive_datetime,
};
use app_macro::Dao;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: String,
    pub role_id: i32,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub expire: NaiveDateTime,
}

impl From<UserRoleDao> for UserRole {
    fn from(dao: UserRoleDao) -> Self {
        Self {
            user_id: dao.user_id,
            role_id: dao.role_id,
            expire: dao.expire,
        }
    }
}

impl UserRole {
    pub async fn find_by_user(user_id: &str) -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper().eq("user_id", user_id);
        let all = UserRoleDao::find_list(&w).await?;
        let all: Vec<Self> = all.into_iter().map(|v| v.into()).collect();
        Ok(all)
    }
    pub async fn find_by_role(role_id: i32) -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper().eq("role_id", role_id);
        let all = UserRoleDao::find_list(&w).await?;
        let all: Vec<Self> = all.into_iter().map(|v| v.into()).collect();
        Ok(all)
    }
    pub async fn find_one(user_id: &str, role_id: i32) -> Result<Self, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", user_id)
            .and()
            .eq("role_id", role_id).limit(1);
        POOL.fetch_by_wrapper::<UserRoleDao>(&w).await.map(Into::into)
    }
}
