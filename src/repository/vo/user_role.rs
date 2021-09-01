use crate::{repository::dao::UserRoleDao, util::datetime_format::naive_datetime};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: Option<String>,
    pub role_id: Option<i32>,
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
