use crate::{
    repository::{
        dao::{org::IntoVecOfVo, Org},
        vo, DBError, Dao, POOL,
    },
    util::now,
};
use rbatis::{
    crud::CRUD,
    plugin::page::{Page, PageRequest},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewOrg {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub domain_id: String,
    #[serde(skip_deserializing)]
    pub created_by: Option<String>,
}

impl NewOrg {
    pub async fn create(self) -> Result<Org, DBError> {
        let id = Uuid::new_v4().to_string();
        let dao = Org {
            id: id.clone(),
            name: self.name,
            description: self.description,
            domain_id: self.domain_id,
            is_deleted: 0,
            created_by: self.created_by.clone(),
            updated_by: self.created_by,
            created_at: now(),
            updated_at: now(),
        };
        Org::create_one(&dao).await?;
        Ok(dao)
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateOrg {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub updated_by: Option<String>,
}

impl UpdateOrg {
    pub async fn save(self, id: &str) -> Result<Org, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = Org::find_one(&w).await?;
        if let Some(name) = self.name {
            dao.name = name;
        }
        dao.description = self.description;
        Org::update_one(&dao, &w).await?;
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct QueryOrg {
    key: Option<String>,
    domain_id: Option<String>,
    #[validate(range(min = 1))]
    page: Option<u64>,
    #[validate(range(min = 1))]
    limit: Option<u64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl QueryOrg {
    pub async fn find_all(self) -> Result<Page<vo::Org>, DBError> {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(10);
        let req = PageRequest::new(page, limit);
        let sort_by = self.sort_by.unwrap_or("created_at".to_string());
        let sort_order = self.sort_order.unwrap_or("DESC".to_string());
        let mut w = POOL.new_wrapper();
        if let Some(domain_id) = self.domain_id {
            w = w.eq("domain_id", &domain_id)
        }
        if let Some(key) = self.key {
            if key != "" {
                w = w.and().like("name", key);
            }
        }
        w = w.order_by(&sort_order.to_uppercase() == "ASC", &[&sort_by]);
        let ret = POOL.fetch_page_by_wrapper::<Org>(&w, &req).await?;

        let records = ret.records.into_vo().await?;
        Ok(Page::<vo::Org> {
            records,
            total: ret.total,
            pages: ret.pages,
            page_no: ret.page_no,
            page_size: ret.page_size,
            search_count: ret.search_count,
        })
    }
}
