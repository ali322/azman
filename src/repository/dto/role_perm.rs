use crate::repository::{dao::RolePermDao, vo::RolePerm, DBError, POOL};
use app_macro::Dao;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleGrantPerm {
    pub role_id: Option<i32>,
    pub perm_id: Option<i32>,
}

impl RoleGrantPerm {
    pub async fn save(&self) -> Result<RolePerm, DBError> {
        let dao = RolePermDao {
            role_id: self.role_id.clone(),
            perm_id: self.perm_id.clone(),
        };
        RolePermDao::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleRevokePerm {
    pub role_id: Option<i32>,
    pub perm_id: Option<i32>,
}

impl RoleRevokePerm {
    pub async fn save(&self) -> Result<RolePerm, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("perm_id", self.perm_id.clone())
            .and()
            .eq("role_id", self.role_id.clone());
        RolePermDao::delete_one(&w).await?;
        RolePermDao::find_one(&w).await.map(Into::into)
    }
}
