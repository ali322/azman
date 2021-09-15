use axum::{handler::post, routing::BoxRoute, Json, Router};
use validator::Validate;

use crate::{
    repository::{
        dao::User,
        dto::{ConnectUser, LoginUser, NewUser},
    },
    util::{
        jwt::{self, Auth},
        APIResult,
    },
};

async fn register(Json(body): Json<NewUser>) -> APIResult {
    body.validate()?;
    if User::find_by_username(&body.username).await.is_ok() {
        return Err(reject!("用户已存在"));
    }
    let user = body.create().await?;
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        is_admin: false,
    });
    Ok(reply!({
      "token": token, "user": user
    }))
}

async fn login(Json(body): Json<LoginUser>) -> APIResult {
    body.validate()?;
    let user_dao = match User::find_by_username_or_email(&body.username_or_email).await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user_dao.password) {
        return Err(reject!("密码不正确"));
    }
    if user_dao.is_actived == 0 {
        return Err(reject!("用户被禁用"));
    }

    let user = body.login(&user_dao).await?;
    let is_admin = user.sys_role.clone().unwrap() == "admin";

    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        is_admin,
    });
    Ok(reply!({
      "token": token, "user": user
    }))
}

async fn connect(Json(body): Json<ConnectUser>) -> APIResult {
    body.validate()?;
    let user: User;
    if let Ok(val) = User::find_by_username(&body.username).await {
        user = val.into();
    } else {
        user = body.create().await?;
    };
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        is_admin: false,
    });
    Ok(reply!({
      "token": token, "user": user
    }))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/register", post(register))
        .route("/login", post(login))
        .route("/connect", post(connect))
        .boxed()
}
