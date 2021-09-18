use crate::{
    repository::{
        dao::{org, perm, role, Org, Perm, Role, Domain, RolePerm, User, UserOrg, UserRole},
        dto::Access,
        vo, Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use axum::{
    extract::{Extension, Path},
    handler::{get, post},
    routing::BoxRoute,
    Json, Router,
};
use std::collections::HashMap;
use tower_http::auth::RequireAuthorizationLayer;

async fn access(Json(body): Json<Access>, Extension(auth): Extension<Auth>) -> APIResult {
    let role_perms = RolePerm::find_by_role(&body.role_id).await?;
    let mut perm_map: HashMap<String, bool> = HashMap::new();
    let perms = Perm::find_by_ids(body.perm_id, None).await?;
    for perm in perms.into_iter() {
        perm_map.insert(
            perm.name.clone(),
            role_perms.iter().any(|v| v.perm_id == perm.id),
        );
    }
    let user_role = UserRole::find_by_id(&auth.id, &body.role_id).await;
    if user_role.is_err() {
        for (_, item) in perm_map.iter_mut() {
            *item = false
        }
    }
    Ok(reply!(perm_map))
}

async fn roles_of_user(Path(id): Path<String>, Extension(_): Extension<Auth>) -> APIResult {
    use role::IntoVecOfVo;
    match User::find_by_id(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("用户 {} 不存在", &id))),
    };
    let user_roles = UserRole::find_by_user(&id).await?;
    let role_ids: Vec<String> = user_roles.into_iter().map(|v| v.role_id).collect();
    let roles: Vec<vo::Role> = if role_ids.len() > 0 {
        Role::find_by_ids(role_ids).await?.into_vo().await?
    } else {
        vec![]
    };
    Ok(reply!(roles))
}

async fn users_of_role(Path(id): Path<String>) -> APIResult {
    match Role::find_by_id(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("角色 {} 不存在", &id))),
    };
    let user_roles = UserRole::find_by_role(&id).await?;
    let user_ids: Vec<String> = user_roles.into_iter().map(|v| v.user_id).collect();
    let users = if user_ids.len() > 0 {
        User::find_by_ids(user_ids).await?
    } else {
        vec![]
    };
    Ok(reply!(users))
}

async fn perms_of_role(Path(id): Path<String>, Extension(_): Extension<Auth>) -> APIResult {
    use perm::IntoVecOfVo;
    match Role::find_by_id(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("角色 {} 不存在", &id))),
    };
    let role_perms = RolePerm::find_by_role(&id).await?;
    let perm_ids: Vec<String> = role_perms.into_iter().map(|v| v.perm_id).collect();
    let perms: Vec<vo::Perm> = if perm_ids.len() > 0 {
        Perm::find_by_ids(perm_ids, None).await?.into_vo().await?
    } else {
        vec![]
    };
    Ok(reply!(perms))
}

async fn orgs_of_user(Path(id): Path<String>, Extension(_): Extension<Auth>) -> APIResult {
    use org::IntoVecOfVo;
    match User::find_by_id(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("用户 {} 不存在", &id))),
    };
    let user_orgs = UserOrg::find_by_user(&id).await?;
    let org_ids: Vec<String> = user_orgs.iter().map(|v| v.org_id.clone()).collect();
    let orgs: Vec<vo::Org> = if org_ids.len() > 0 {
        Org::find_by_ids(org_ids, None).await?.into_vo().await?
    } else {
        vec![]
    };
    Ok(reply!(orgs))
}

async fn users_of_org(Path(id): Path<String>) -> APIResult {
    match Org::find_by_id(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("组织 {} 不存在", &id))),
    };
    let user_orgs = UserOrg::find_by_org(&id).await?;
    let user_ids: Vec<String> = user_orgs.iter().map(|v| v.user_id.clone()).collect();
    let users: Vec<User> = if user_ids.len() > 0 {
        User::find_by_ids(user_ids).await?
    } else {
        vec![]
    };
    Ok(reply!(users))
}

async fn domains_of_user(Path(id): Path<String>, Extension(_): Extension<Auth>) -> APIResult {
    match User::find_by_id(&id).await {
        Ok(_) => (),
        Err(_) => return Err(reject!(format!("用户 {} 不存在", &id))),
    };
    let user_roles = UserRole::find_by_user(&id).await?;
    let role_ids: Vec<String> = user_roles.into_iter().map(|v| v.role_id).collect();
    let domains = Domain::find_by_admin_role(role_ids).await?;
    Ok(reply!(domains))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/access", post(access))
        .route("/user/:id/role", get(roles_of_user))
        .route("/user/:id/org", get(orgs_of_user))
        .route("/role/:id/user", get(users_of_role))
        .route("/role/:id/perm", get(perms_of_role))
        .route("/org/:id/user", get(users_of_org))
        .route("/user/:id/domain", get(domains_of_user))
        .layer(restrict_layer)
        .boxed()
}
