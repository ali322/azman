use crate::repository::{Dao, dao::UserDao, vo::User, DBError, POOL};
use bcrypt::{hash, verify};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
    pub sys_role: Option<String>,
    #[serde(default = "now")]
    pub last_logined_at: NaiveDateTime,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewUser {
    pub async fn create(self) -> Result<User, DBError> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(&self.password, 4).unwrap();
        let dao = UserDao {
            id: id.clone(),
            username: self.username,
            password: hashed_password,
            email: self.email,
            avatar: self.avatar,
            memo: self.memo,
            sys_role: Some("member".to_string()),
            is_actived: Some(true as i32),
            last_logined_at: now(),
            created_at: now(),
        };
        UserDao::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
}

impl UpdateUser {
    pub async fn save(self, id: &str) -> Result<User, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = UserDao::find_one(&w).await?;
        dao.email = self.email;
        dao.avatar = self.avatar;
        dao.memo = self.memo;
        UserDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(length(min = 1, max = 200))]
    pub username_or_email: String,
    #[validate(length(min = 3, max = 100))]
    pub password: String,
}

impl LoginUser {
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(self.password.clone(), target).unwrap()
    }
    pub async fn login(&self, dao: &UserDao) -> Result<User, DBError> {
        let mut dao = dao.to_owned();
        dao.last_logined_at = now();
        let w = POOL.new_wrapper().eq("id", &dao.id);
        UserDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ConnectUser {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
    pub sys_role: Option<String>,
    #[serde(default = "now")]
    pub last_logined_at: NaiveDateTime,
}

const CONNECT_PASSWORD: &'static str = "123456";

impl ConnectUser {
    pub async fn create(self) -> Result<User, DBError> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(CONNECT_PASSWORD, 4).unwrap();
        let dao = UserDao {
            id: id.clone(),
            username: self.username,
            password: hashed_password,
            email: self.email,
            avatar: self.avatar,
            memo: self.memo,
            sys_role: Some("member".to_string()),
            is_actived: Some(true as i32),
            last_logined_at: now(),
            created_at: now(),
        };
        UserDao::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 3, max = 200))]
    pub old_password: String,
    #[validate(length(min = 3, max = 200))]
    pub new_password: String,
    #[validate(must_match(other = "new_password", message = "密码不匹配"))]
    pub repeat_password: String,
}

impl ChangePassword {
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(&self.old_password, target).unwrap()
    }
    pub async fn change_password(&self, dao: &UserDao) -> Result<User, DBError> {
        let mut dao = dao.to_owned();
        let hashed_password = hash(&self.new_password, 4).unwrap();
        dao.password = hashed_password;
        let w = POOL.new_wrapper().eq("id", &dao.id);
        UserDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResetPassword {
    #[validate(length(min = 3, max = 200))]
    pub new_password: String,
    #[validate(must_match = "new_password")]
    pub repeat_password: String,
}

impl ResetPassword {
    pub async fn reset_password(&self, dao: &UserDao) -> Result<User, DBError> {
        let hashed_password = hash(&self.new_password, 4).unwrap();
        let mut dao = dao.to_owned();
        let w = POOL.new_wrapper().eq("id", &dao.id);
        dao.password = hashed_password;
        UserDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}
