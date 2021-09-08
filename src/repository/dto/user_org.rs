use crate::{
    repository::{dao::UserOrg, DBError, Dao, POOL},
    util::{default_expire, now},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserJoinOrg {
    pub user_id: String,
    pub org_id: String,
    pub expire: Option<NaiveDateTime>,
}

impl UserJoinOrg {
    pub async fn save(self) -> Result<UserOrg, DBError> {
        let dao = UserOrg {
            user_id: self.user_id,
            org_id: self.org_id,
            expire: default_expire(),
            created_at: now(),
        };
        UserOrg::create_one(&dao).await?;
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserLeaveOrg {
    pub user_id: String,
    pub org_id: String,
}

impl UserLeaveOrg {
    pub async fn save(self) -> Result<u64, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", self.user_id)
            .and()
            .eq("org_id", self.org_id);
        UserOrg::delete_one(&w).await
    }
}
