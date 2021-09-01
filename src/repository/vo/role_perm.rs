use crate::repository::dao::RolePermDao;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePerm {
    pub role_id: Option<i32>,
    pub perm_id: Option<i32>,
}

impl From<RolePermDao> for RolePerm {
    fn from(dao: RolePermDao) -> Self {
        Self {
            role_id: dao.role_id,
            perm_id: dao.perm_id,
        }
    }
}
