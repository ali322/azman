use crate::{
    repository::{dao::RoleDao, DBError, POOL},
    util::datetime_format::naive_datetime,
};
use app_macro::Dao;
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

impl Role {
    pub async fn find_one(id: i32) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        RoleDao::find_one(&w).await.map(Into::into)
    }
    pub async fn find_all() -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper();
        let all = RoleDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
    pub async fn find_by_ids(
        ids: Vec<i32>,
        domain_id: Option<String>,
    ) -> Result<Vec<Self>, DBError> {
        let mut w = POOL.new_wrapper().r#in("id", &ids);
        if let Some(domain_id) = domain_id {
            w = w.and().eq("domain_id", domain_id);
        }
        let all = RoleDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
    pub async fn delete_one(id: i32) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        RoleDao::delete_one(&w).await?;
        Self::find_one(id as i32).await
    }
}
