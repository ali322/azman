use crate::repository::{DBError, POOL};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "roles")]
#[derive(Debug, Clone)]
pub struct RoleDao {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
    pub level: i32,
    pub domain_id: String,
    pub is_deleted: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}

impl RoleDao {
    pub async fn find_one(w: &Wrapper) -> Result<Self, DBError> {
        let w = w.clone().order_by(true, &["id"]).limit(1);
        POOL.fetch_by_wrapper::<Self>(&w).await
    }
    pub async fn find_list(w: &Wrapper) -> Result<Vec<Self>, DBError> {
        POOL.fetch_list_by_wrapper(w).await
    }
    pub async fn create_one(&self) -> Result<Option<i64>, DBError> {
        let created = POOL.save(&self, &[]).await?;
        Ok(created.last_insert_id)
    }
    pub async fn update_one(&self, w: &Wrapper) -> Result<u64, DBError> {
        POOL.update_by_wrapper(&self, w, &[]).await
    }
    pub async fn delete_one(w: &Wrapper) -> Result<u64, DBError> {
        POOL.remove_by_wrapper::<Self>(w).await
    }
}
