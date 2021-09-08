use crate::repository::{dao::RolePermDao, vo::RolePerm, DBError, POOL};
use serde::{Deserialize, Serialize};
use app_macro_trait::Dao;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleGrantPerm {
    pub role_id: i32,
    pub perm_id: i32,
}

impl RoleGrantPerm {
    pub async fn save(self) -> Result<RolePerm, DBError> {
        let dao = RolePermDao {
            role_id: self.role_id,
            perm_id: self.perm_id,
        };
        RolePermDao::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleRevokePerm {
    pub role_id: i32,
    pub perm_id: i32,
}

impl RoleRevokePerm {
    pub async fn save(self) -> Result<u64, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("perm_id", self.perm_id)
            .and()
            .eq("role_id", self.role_id);
        RolePermDao::delete_one(&w).await
    }
}
