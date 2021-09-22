use crate::{
    repository::{
        dao::{perm::IntoVecOfVo, Perm},
        vo, DBError, Dao, POOL,
    },
    util::{now, uuid_v4},
};
use rbatis::{
    crud::CRUD,
    plugin::page::{Page, PageRequest},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewPerm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
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
    domain_id: Option<String>,
    #[validate(range(min = 1))]
    page: Option<u64>,
    #[validate(range(min = 1))]
    limit: Option<u64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl QueryPerm {
    pub async fn find_all(self) -> Result<Page<vo::Perm>, DBError> {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(10);
        let req = PageRequest::new(page, limit);
        let sort_by = self.sort_by.unwrap_or("created_at".to_string());
        let sort_order = self.sort_order.unwrap_or("DESC".to_string());
        let mut w = POOL.new_wrapper();
        if let Some(domain_id) = self.domain_id {
            let domain_ids: Vec<&str> = domain_id.split(",").into_iter().collect();
            w = w.r#in("domain_id", &domain_ids);
        }
        if let Some(key) = self.key {
            if key != "" {
                w = w.and().like("name", key);
            }
        }
        w = w.order_by(&sort_order.to_uppercase() == "ASC", &[&sort_by]);
        let ret = POOL.fetch_page_by_wrapper::<Perm>(&w, &req).await?;

        let records = ret.records.into_vo().await?;
        Ok(Page::<vo::Perm> {
            records,
            total: ret.total,
            pages: ret.pages,
            page_no: ret.page_no,
            page_size: ret.page_size,
            search_count: ret.search_count,
        })
    }
}
