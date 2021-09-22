use super::Domain;
use crate::{
    repository::{vo, DBError, Dao, POOL},
    util::serde_format::{i32_bool, naive_datetime},
};
use app_macro::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};
use serde::Serialize;
use std::collections::HashMap;

#[crud_table(table_name: "orgs")]
#[derive(Debug, Clone, Dao)]
pub struct Org {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub domain_id: String,
    #[serde(serialize_with = "i32_bool::serialize")]
    pub is_deleted: i32,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl Org {
    pub async fn find_by_ids(
        id: Vec<String>,
        domain_id: Option<String>,
    ) -> Result<Vec<Self>, DBError> {
        let mut w = POOL.new_wrapper().r#in("id", &id);
        if let Some(domain_id) = domain_id {
            w = w.eq("domain_id", domain_id);
        }
        Self::find_list(&w).await
    }
    pub async fn find_all(domain_ids: Vec<String>) -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper().r#in("domain_id", &domain_ids);
        Self::find_list(&w).await
    }
}

#[async_trait]
pub trait IntoVecOfVo {
    async fn into_vo(&self) -> Result<Vec<vo::Org>, DBError>;
}

#[async_trait]
impl IntoVecOfVo for Vec<Org> {
    async fn into_vo(&self) -> Result<Vec<vo::Org>, DBError> {
        let domain_ids: Vec<String> = self.iter().map(|v| v.domain_id.clone()).collect();
        let w = POOL.new_wrapper().r#in("id", &domain_ids);
        let domains = POOL.fetch_list_by_wrapper::<Domain>(&w).await?;
        let mut domain_map = HashMap::new();
        for domain in domains {
            domain_map.insert(domain.id.clone(), domain.clone());
        }
        let mut records: Vec<vo::Org> = self.iter().map(|v| vo::Org::from(v.clone())).collect();
        for mut r in &mut records {
            let domain = domain_map.get(&r.domain_id).cloned();
            r.domain = domain.map(Into::into);
        }
        Ok(records)
    }
}
