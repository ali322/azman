use std::collections::HashMap;

use axum::{
    extract::{Extension, Path},
    handler::{get, post},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dto::Access,
        vo::{Perm, Role, RolePerm, User, UserRole},
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};

async fn access(Json(body): Json<Access>, Extension(auth): Extension<Auth>) -> APIResult {
    let role_perms = RolePerm::find_by_role(body.role_id).await?;
    let mut perm_map: HashMap<String, bool> = HashMap::new();
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let perms = Perm::find_by_ids(body.perm_id, domain_id).await?;
    for perm in perms.into_iter() {
        perm_map.insert(
            perm.name.clone(),
            role_perms.iter().any(|v| v.perm_id == perm.id.unwrap()),
        );
    }
    let user_role = UserRole::find_one(auth.id, body.role_id).await;
    if user_role.is_err() {
        for (_, item) in perm_map.iter_mut() {
            *item = false
        }
    }
    Ok(reply!(perm_map))
}

async fn roles_of_user(Path(id): Path<String>, Extension(auth): Extension<Auth>) -> APIResult {
    match User::find_one(id.clone()).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("用户 {} 不存在", id.clone()))),
    };
    let user_roles = UserRole::find_by_user(id).await?;
    let role_ids: Vec<i32> = user_roles.iter().map(|v| v.role_id).collect();
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let roles = Role::find_by_ids(role_ids, domain_id).await?;
    Ok(reply!(roles))
}

async fn users_of_role(Path(id): Path<i32>) -> APIResult {
    match Role::find_one(id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("角色 {} 不存在", id))),
    };
    let user_roles = UserRole::find_by_role(id).await?;
    let user_ids: Vec<String> = user_roles.iter().map(|v| v.user_id.clone()).collect();
    let users = User::find_by_ids(user_ids).await?;
    Ok(reply!(users))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/access", post(access))
        .route("/user/:id/role", get(roles_of_user))
        .route("/role/:id/user", get(users_of_role))
        .layer(restrict_layer)
        .boxed()
}
