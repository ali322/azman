use crate::{
    repository::{dao::OrgDao, DBError, POOL},
    util::datetime_format::naive_datetime,
};
use app_macro::Dao;
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
    pub created_by: String,
    pub updated_by: String,
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

impl Org {
    pub async fn find_one(id: String) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        OrgDao::find_one(&w).await.map(Into::into)
    }
    pub async fn find_all() -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper();
        let all = OrgDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
    pub async fn find_by_ids(
        ids: Vec<String>,
        domain_id: Option<String>,
    ) -> Result<Vec<Self>, DBError> {
        let mut w = POOL.new_wrapper().r#in("id", &ids);
        if let Some(domain_id) = domain_id {
            w = w.and().eq("domain_id", domain_id);
        }
        let all = OrgDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
    pub async fn delete_one(id: String) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("id", id.clone());
        OrgDao::delete_one(&w).await?;
        Self::find_one(id).await
    }
}
