use crate::repository::{Dao, dao::UserOrgDao, vo::UserOrg, DBError, POOL};
use chrono::{Duration, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserJoinOrg {
    pub user_id: String,
    pub org_id: String,
    pub expire: Option<NaiveDateTime>,
}

fn default_expire() -> NaiveDateTime {
    Local::now().naive_local() + Duration::days(30)
}

impl UserJoinOrg {
    pub async fn save(self) -> Result<UserOrg, DBError> {
        let dao = UserOrgDao {
            user_id: self.user_id,
            org_id: self.org_id,
            expire: default_expire(),
        };
        UserOrgDao::create_one(&dao).await?;
        Ok(dao.into())
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
        UserOrgDao::delete_one(&w).await
    }
}
