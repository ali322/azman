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
        vo::{Org, Perm, Role, RolePerm, User, UserOrg, UserRole},
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
    let user_role = UserRole::find_one(&auth.id, body.role_id).await;
    if user_role.is_err() {
        for (_, item) in perm_map.iter_mut() {
            *item = false
        }
    }
    Ok(reply!(perm_map))
}

async fn roles_of_user(Path(id): Path<String>, Extension(auth): Extension<Auth>) -> APIResult {
    match User::find_one(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("用户 {} 不存在", &id))),
    };
    let user_roles = UserRole::find_by_user(&id).await?;
    let role_ids: Vec<i32> = user_roles.iter().map(|v| v.role_id).collect();
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let roles = if role_ids.len() > 0 {
        Role::find_by_ids(role_ids, domain_id).await?
    } else {
        vec![]
    };
    Ok(reply!(roles))
}

async fn users_of_role(Path(id): Path<i32>) -> APIResult {
    match Role::find_one(id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("角色 {} 不存在", id))),
    };
    let user_roles = UserRole::find_by_role(id).await?;
    let user_ids: Vec<String> = user_roles.iter().map(|v| v.user_id.clone()).collect();
    let users = if user_ids.len() > 0 {
        User::find_by_ids(user_ids).await?
    } else {
        vec![]
    };
    Ok(reply!(users))
}

async fn perms_of_role(Path(id): Path<i32>, Extension(auth): Extension<Auth>) -> APIResult {
    match Role::find_one(id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("角色 {} 不存在", id))),
    };
    let role_perms = RolePerm::find_by_role(id).await?;
    let perm_ids: Vec<i32> = role_perms.iter().map(|v| v.perm_id).collect();
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let perms = if perm_ids.len() > 0 {
        Perm::find_by_ids(perm_ids, domain_id).await?
    } else {
        vec![]
    };
    Ok(reply!(perms))
}

async fn orgs_of_user(Path(id): Path<String>, Extension(auth): Extension<Auth>) -> APIResult {
    match User::find_one(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("用户 {} 不存在", &id))),
    };
    let user_orgs = UserOrg::find_by_user(&id).await?;
    let org_ids: Vec<String> = user_orgs.iter().map(|v| v.org_id.clone()).collect();
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let orgs = if org_ids.len() > 0 {
        Org::find_by_ids(org_ids, domain_id).await?
    } else {
        vec![]
    };
    Ok(reply!(orgs))
}

async fn users_of_org(Path(id): Path<String>) -> APIResult {
    match Org::find_one(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("组织 {} 不存在", &id))),
    };
    let user_orgs = UserOrg::find_by_org(&id).await?;
    let user_ids: Vec<String> = user_orgs.iter().map(|v| v.user_id.clone()).collect();
    let users = if user_ids.len() > 0 {
        User::find_by_ids(user_ids).await?
    } else {
        vec![]
    };
    Ok(reply!(users))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/access", post(access))
        .route("/user/:id/role", get(roles_of_user))
        .route("/user/:id/org", get(orgs_of_user))
        .route("/role/:id/user", get(users_of_role))
        .route("/role/:id/perm", get(perms_of_role))
        .route("/org/:id/user", get(users_of_org))
        .layer(restrict_layer)
        .boxed()
}
