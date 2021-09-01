use crate::{repository::dao::UserOrgDao, util::datetime_format::naive_datetime};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOrg {
    pub user_id: Option<String>,
    pub org_id: Option<String>,
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
