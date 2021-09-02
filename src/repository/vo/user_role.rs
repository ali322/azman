use crate::{repository::{dao::UserRoleDao, POOL, DBError}, util::datetime_format::naive_datetime};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use app_macro::Dao;

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
  pub async fn find_by_user(user_id: String) -> Result<Vec<Self>, DBError> {
      let w = POOL.new_wrapper().eq("user_id", user_id);
      let all = UserRoleDao::find_list(&w).await?;
      let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
      Ok(all)
  }
}