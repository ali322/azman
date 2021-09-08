use crate::{repository::dao::OrgDao, util::datetime_format::naive_datetime};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Org {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub domain_id: String,
    pub is_deleted: Option<bool>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl From<OrgDao> for Org {
    fn from(dao: OrgDao) -> Self {
        Self {
            id: dao.id,
            name: dao.name,
            description: dao.description,
            domain_id: dao.domain_id,
            is_deleted: dao.is_deleted.map(|v| v == 1),
            created_by: dao.created_by,
            updated_by: dao.updated_by,
            created_at: dao.created_at,
            updated_at: dao.updated_at,
        }
    }
}
