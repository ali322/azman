use crate::repository::{DBError, POOL};
use chrono::NaiveDateTime;
use app_macro::Dao;
use app_macro_derive::Dao;
use async_trait::async_trait;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "user_has_orgs")]
#[derive(Debug, Clone, Dao)]
pub struct UserOrgDao {
    pub user_id: String,
    pub org_id: String,
    pub expire: NaiveDateTime
}
