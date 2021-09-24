use crate::{
    repository::{dao::UserOrg, DBError, Dao, POOL},
    util::{default_expire, now},
};
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserJoinOrg {
    pub user_ids: Vec<String>,
    pub org_id: String,
    pub expire: Option<NaiveDateTime>,
}

impl UserJoinOrg {
    pub async fn save(self) -> Result<Vec<UserOrg>, DBError> {
        let user_orgs: Vec<UserOrg> = self
            .user_ids
            .iter()
            .map(|user_id| UserOrg {
                org_id: self.org_id.clone(),
                user_id: user_id.clone(),
                created_at: now(),
                expire: default_expire(),
            })
            .collect();
        POOL.save_batch(&user_orgs, &[]).await?;
        Ok(user_orgs)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserLeaveOrg {
    pub user_ids: Vec<String>,
    pub org_id: String,
}

impl UserLeaveOrg {
    pub async fn save(self) -> Result<u64, DBError> {
        let w = POOL
            .new_wrapper()
            .r#in("user_id", &self.user_ids)
            .and()
            .eq("org_id", self.org_id);
        UserOrg::delete_one(w).await
    }
}
