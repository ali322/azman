use crate::{
    repository::{dao::Org, DBError, Dao, POOL, vo},
    util::now,
};
use rbatis::plugin::page::{Page, PageRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewOrg {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
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
    #[validate(range(min = 1))]
    page: Option<u64>,
    #[validate(range(min = 1))]
    limit: Option<u64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl QueryOrg {
    pub async fn find_all(self, domain_id: Option<String>) -> Result<Page<vo::Org>, DBError> {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(10);
        let req = PageRequest::new(page, limit);
        let sort_by = self.sort_by.unwrap_or("created_at".to_string());
        let sort_order = self.sort_order.unwrap_or("DESC".to_string());
        if let Some(domain_id) = domain_id {
            let ret = find_page_by_domain(&req, &domain_id, &sort_by, &sort_order).await?;
            Ok(ret)
        } else {
            let ret = find_page(&req, &sort_by, &sort_order).await?;
            Ok(ret)
        }
    }
}

#[py_sql(
    POOL,
    "select r.id, r.name, r.is_deleted, r.created_at, r.updated_at, d.id as `domain.id` , d.name as `domain.name` 
from roles r left join domains d on d.id = r.domain_id 
where r.domain_id = #{domain_id} order by r.${sort_by} ${sort_order}"
)]
async fn find_page_by_domain(
    page_req: &PageRequest,
    domain_id: &str,
    sort_by: &str,
    sort_order: &str,
) -> Page<vo::Org> {
}

#[py_sql(
    POOL,
    "select r.id, r.name, r.is_deleted, r.created_at, r.updated_at, d.id as `domain_id` , d.name as `domain_name` 
from roles r left join domains d on d.id = r.domain_id order by r.${sort_by} ${sort_order}"
)]
async fn find_page(page_req: &PageRequest, sort_by: &str, sort_order: &str) -> Page<vo::Org> {}