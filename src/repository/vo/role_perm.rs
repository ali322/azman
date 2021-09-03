use crate::repository::{dao::RolePermDao, DBError, POOL};
use app_macro::Dao;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePerm {
    pub role_id: i32,
    pub perm_id: i32,
}

impl From<RolePermDao> for RolePerm {
    fn from(dao: RolePermDao) -> Self {
        Self {
            role_id: dao.role_id,
            perm_id: dao.perm_id,
        }
    }
}

impl RolePerm {
    pub async fn find_by_role(role_id: i32) -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper().eq("role_id", role_id);
        let all = RolePermDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
}
