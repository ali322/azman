use crate::{
    repository::{dao::Perm, DBError, Dao, POOL, vo},
    util::{now, uuid_v4},
};
use rbatis::plugin::page::{Page, PageRequest};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewPerm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
    #[serde(skip_deserializing)]
    pub domain_id: String,
    #[serde(skip_deserializing)]
    pub created_by: Option<String>,
}

impl NewPerm {
    pub async fn create(self) -> Result<Perm, DBError> {
        let id = uuid_v4();
        let dao = Perm {
            id,
            name: self.name,
            description: self.description,
            value: self.value,
            domain_id: self.domain_id,
            is_deleted: 0,
            created_by: self.created_by.clone(),
            updated_by: self.created_by,
            created_at: now(),
            updated_at: now(),
        };
        Perm::create_one(&dao).await?;
        Ok(dao)
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdatePerm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
    #[serde(skip_deserializing)]
    pub updated_by: Option<String>,
}

impl UpdatePerm {
    pub async fn save(self, id: &str) -> Result<Perm, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = Perm::find_one(&w).await?;
        dao.name = self.name;
        dao.description = self.description;
        dao.value = self.value;
        dao.updated_by = self.updated_by;
        Perm::update_one(&dao, &w).await?;
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct QueryPerm {
    key: Option<String>,
    #[validate(range(min = 1))]
    page: Option<u64>,
    #[validate(range(min = 1))]
    limit: Option<u64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl QueryPerm {
    pub async fn find_all(self, domain_id: Option<String>) -> Result<Page<vo::Perm>, DBError> {
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
    "select r.id, r.name, r.value, r.is_deleted, r.created_at, r.updated_at, d.id as `domain.id` , d.name as `domain.name` 
from perms r left join domains d on d.id = r.domain_id 
where r.domain_id = #{domain_id} order by r.${sort_by} ${sort_order}"
)]
async fn find_page_by_domain(
    page_req: &PageRequest,
    domain_id: &str,
    sort_by: &str,
    sort_order: &str,
) -> Page<vo::Perm> {
}

#[py_sql(
    POOL,
    "select r.id, r.name, r.value, r.is_deleted, r.created_at, r.updated_at, d.id as `domain_id` , d.name as `domain_name` 
from perms r left join domains d on d.id = r.domain_id order by r.${sort_by} ${sort_order}"
)]
async fn find_page(page_req: &PageRequest, sort_by: &str, sort_order: &str) -> Page<vo::Perm> {}