use crate::{repository::{Dao, dao::RolePerm, DBError, POOL}, util::now};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleGrantPerm {
    pub role_id: String,
    pub perm_id: String,
}

impl RoleGrantPerm {
    pub async fn save(self) -> Result<RolePerm, DBError> {
        let dao = RolePerm {
            role_id: self.role_id,
            perm_id: self.perm_id,
            created_at: now()
        };
        RolePerm::create_one(&dao).await?;
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleRevokePerm {
    pub role_id: String,
    pub perm_id: String,
}

impl RoleRevokePerm {
    pub async fn save(self) -> Result<u64, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("perm_id", self.perm_id)
            .and()
            .eq("role_id", self.role_id);
        RolePerm::delete_one(&w).await
    }
}
