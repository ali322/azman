use crate::{
    repository::{dao::UserOrgDao, DBError, POOL},
    util::datetime_format::naive_datetime,
};
use app_macro::Dao;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOrg {
    pub user_id: String,
    pub org_id: String,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub expire: NaiveDateTime,
}

impl From<UserOrgDao> for UserOrg {
    fn from(dao: UserOrgDao) -> Self {
        Self {
            user_id: dao.user_id,
            org_id: dao.org_id,
            expire: dao.expire,
        }
    }
}

impl UserOrg {
    pub async fn find_by_user(user_id: String) -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper().eq("user_id", user_id);
        let all = UserOrgDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
    pub async fn find_by_org(org_id: String) -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper().eq("org_id", org_id);
        let all = UserOrgDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
}
