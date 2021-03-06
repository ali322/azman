use crate::{
    repository::{dao::RolePerm, DBError, Dao, POOL},
    util::now,
};
use rbatis::crud::{CRUD, CRUDMut};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleGrantPerm {
    pub role_id: String,
    pub perm_ids: Vec<String>,
}

impl RoleGrantPerm {
    pub async fn save(self) -> Result<Vec<RolePerm>, DBError> {
        let role_perms: Vec<RolePerm> = self
            .perm_ids
            .iter()
            .map(|perm_id| RolePerm {
                role_id: self.role_id.clone(),
                perm_id: perm_id.clone(),
                created_at: now(),
            })
            .collect();
        POOL.save_batch(&role_perms, &[]).await?;
        Ok(role_perms)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RoleRevokePerm {
    pub role_id: String,
    pub perm_ids: Vec<String>,
}

impl RoleRevokePerm {
    pub async fn save(self) -> Result<u64, DBError> {
        let w = POOL
            .new_wrapper()
            .r#in("perm_id", &self.perm_ids)
            .and()
            .eq("role_id", self.role_id);
        RolePerm::delete_one(w).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleChangePerm {
    pub perm_ids: Vec<String>,
    pub role_id: String,
}

impl RoleChangePerm {
    pub async fn save(self) -> Result<Vec<RolePerm>, DBError> {
        let mut tx = POOL.acquire_begin().await.unwrap();
        let w = POOL.new_wrapper().eq("role_id", &self.role_id);
        tx.remove_by_wrapper::<RolePerm>(w).await?;
        let rows: Vec<RolePerm> = self.perm_ids
            .iter()
            .map(|perm_id| RolePerm {
                role_id: self.role_id.clone(),
                perm_id: perm_id.clone(),
                created_at: now(),
            })
            .collect();
        tx.save_batch(&rows, &[]).await?;
        tx.commit().await.unwrap();
        Ok(rows)
    }
}
