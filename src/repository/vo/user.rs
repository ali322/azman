use crate::{repository::dao::UserDao, util::datetime_format::naive_datetime};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_deserializing)]
    pub password: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
    pub sys_role: Option<String>,
    pub is_actived: Option<bool>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub last_logined_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl From<UserDao> for User {
    fn from(u: UserDao) -> Self {
        Self {
            id: u.id,
            username: u.username,
            password: u.password,
            email: u.email,
            avatar: u.avatar,
            memo: u.memo,
            sys_role: u.sys_role,
            is_actived: u.is_actived,
            last_logined_at: u.last_logined_at,
            created_at: u.created_at,
        }
    }
}
