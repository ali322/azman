use rbatis::crud::CRUD;
use serde::{Serialize, Deserialize};
use validator::Validate;
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use bcrypt::{hash};
use crate::repository::{POOL, DBError, dao::UserDao, vo::User};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(length(min = 1, max = 100))]
    pub password: String,
    #[validate(must_match(other = "password", message = "密码不匹配"))]
    pub repeat_password: String,
    #[validate(email)]
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
    #[serde(default = "now")]
    pub last_logined_at: NaiveDateTime,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewUser {
    pub async fn exists(&self) -> Result<UserDao, DBError> {
        let w = POOL.new_wrapper().eq("username", self.username.clone());
        UserDao::find_one(&w).await
    }
    pub async fn create(&self) -> Result<User, DBError> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(&self.password, 4).unwrap();
        let dao = UserDao {
            id: id.clone(),
            username: self.username.clone(),
            password: hashed_password,
            email: self.email.clone(),
            avatar: self.avatar.clone(),
            memo: self.memo.clone(),
            is_actived: None,
            sys_role: None,
            last_logined_at: now(),
            created_at: now(),
        };
        POOL.save(&dao, &[]).await?;
        User::find_one(id).await
    }
}