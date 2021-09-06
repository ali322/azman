use crate::{
    repository::{dao::DomainDao, DBError, POOL},
    util::datetime_format::naive_datetime,
};
use app_macro::Dao;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub default_role_id: Option<i32>,
    pub admin_role_id: Option<i32>,
    pub is_deleted: Option<bool>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
}

impl From<DomainDao> for Domain {
    fn from(dao: DomainDao) -> Self {
        Self {
            id: dao.id,
            name: dao.name,
            description: dao.description,
            default_role_id: dao.default_role_id,
            admin_role_id: dao.admin_role_id,
            is_deleted: dao.is_deleted.map(|v| v == 1),
            created_at: dao.created_at,
            updated_at: dao.updated_at,
        }
    }
}

impl Domain {
    pub async fn find_one(id: &str) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        DomainDao::find_one(&w).await.map(Into::into)
    }
    pub async fn find_all() -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper();
        let all = DomainDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
    pub async fn delete_one(id: &str) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        DomainDao::delete_one(&w).await?;
        Self::find_one(id).await
    }
}
