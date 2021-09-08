use crate::{
    repository::{
        dao::User,
        dto::{ChangePassword, ResetPassword, UpdateUser},
        Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use axum::{
    extract::{Extension, Path},
    handler::{get, post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;
use validator::Validate;

async fn all() -> APIResult {
    let all: Vec<User> = User::find_all()
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one: User = User::find_by_id(&id).await?.into();
    Ok(reply!(one))
}

async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>) -> APIResult {
    body.validate()?;
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

async fn change_password(
    Json(body): Json<ChangePassword>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    body.validate()?;
    let user = match User::find_by_id(&auth.id).await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user.password) {
        return Err(reject!("旧密码不正确"));
    }
    let user = body.change_password(&user).await?;
    Ok(reply!(user))
}

async fn reset_password(
    id: String,
    Json(body): Json<ResetPassword>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    if !auth.is_admin {
        return Err(reject!("仅管理员可访问"));
    }
    body.validate()?;
    let user = match User::find_by_id(&id).await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    let user = body.reset_password(&user).await?;
    Ok(reply!(user))
}

async fn me(Extension(auth): Extension<Auth>) -> APIResult {
    Ok(reply!(auth))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/user", get(all))
        .route("/user/:id", put(update).get(one))
        .route("/change/password", post(change_password))
        .route("/reset/:id/password", post(reset_password))
        .route("/me", get(me))
        .layer(restrict_layer)
        .boxed()
}
