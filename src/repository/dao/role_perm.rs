use crate::repository::{DBError, POOL};
use rbatis::{crud::CRUD, wrapper::Wrapper};
use app_macro::Dao;
use app_macro_derive::Dao;
use async_trait::async_trait;

#[crud_table(table_name: "role_has_perms")]
#[derive(Debug, Clone, Dao)]
pub struct RolePermDao {
    pub role_id: i32,
    pub perm_id: i32
}
