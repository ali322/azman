use crate::{repository::dao::RoleDao, util::datetime_format::naive_datetime};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
    pub level: i32,
    pub domain_id: String,
    pub is_deleted: Option<bool>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl From<RoleDao> for Role {
    fn from(dao: RoleDao) -> Self {
        Self {
            id: dao.id,
            name: dao.name,
            description: dao.description,
            value: dao.value,
            level: dao.level,
            domain_id: dao.domain_id,
            is_deleted: dao.is_deleted.map(|v| v == 1),
            created_by: dao.created_by,
            updated_by: dao.updated_by,
            created_at: dao.created_at,
            updated_at: dao.updated_at,
        }
    }
}
