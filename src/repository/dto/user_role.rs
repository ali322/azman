use crate::{
    repository::{
        dao::{Role, User, UserRole},
        DBError, Dao, POOL,
    },
    util::{default_expire, now, serde_format::naive_datetime},
};
use chrono::NaiveDateTime;
use rbatis::crud::CRUDMut;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserGrantRole {
    pub user_id: String,
    pub role_id: String,
    #[serde(skip_deserializing)]
    pub role_level: i32,
}

impl UserGrantRole {
    pub async fn save(self) -> Result<UserRole, DBError> {
        let dao = UserRole {
            user_id: self.user_id,
            role_id: self.role_id,
            role_level: self.role_level,
            expire: default_expire(),
            created_at: now(),
        };
        UserRole::create_one(&dao).await?;
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserRole {
    pub user_id: String,
    pub role_id: String,
    #[serde(with = "naive_datetime")]
    pub expire: NaiveDateTime,
}

impl UpdateUserRole {
    pub async fn save(self) -> Result<UserRole, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", self.user_id)
            .and()
            .eq("role_id", self.role_id);
        let mut dao = UserRole::find_one(&w).await?;
        dao.expire = self.expire;
        UserRole::update_one(&dao, &w).await?;
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserRevokeRole {
    pub user_id: String,
    pub role_id: String,
}

impl UserRevokeRole {
    pub async fn save(self) -> Result<u64, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("user_id", self.user_id)
            .and()
            .eq("role_id", self.role_id);
        UserRole::delete_one(&w).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChangeRole {
    pub user_ids: Vec<String>,
    pub role_id: String,
}

impl UserChangeRole {
    pub async fn save(self, role: Role, users: Vec<User>) -> Result<Vec<UserRole>, DBError> {
        let mut tx = POOL.acquire_begin().await.unwrap();
        let w = POOL.new_wrapper().eq("role_id", &self.role_id);
        tx.remove_by_wrapper::<UserRole>(&w).await?;
        let rows: Vec<UserRole> = users
            .into_iter()
            .map(|user| UserRole {
                role_id: self.role_id.clone(),
                user_id: user.id,
                role_level: role.level,
                expire: default_expire(),
                created_at: now(),
            })
            .collect();
        tx.save_batch(&rows, &[]).await?;
        tx.commit().await.unwrap();
        Ok(rows)
    }
}
