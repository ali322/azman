use axum::{
    extract::{Extension, Path, Query},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dao::{Domain, Role, User, UserRole},
        dto::{
            NewRole, QueryRole, UpdateRole, UpdateUserRole, UserChangeRole, UserGrantRole,
            UserRevokeRole,
        },
        Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use validator::Validate;

async fn all(Query(q): Query<QueryRole>) -> APIResult {
    let all = q.find_all().await?;
    // let all: Vec<Role> = Role::find_all(domain_id).await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one: Role = Role::find_by_id(&id).await?.into();
    Ok(reply!(one))
}

async fn create(Json(mut body): Json<NewRole>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain = match Domain::find_by_id(&body.domain_id).await {
        Ok(val) => val,
        Err(_) => return Err(reject!(format!("来源域 {} 不存在", &body.domain_id))),
    };
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    body.validate()?;
    body.created_by = Some(auth.id.clone());
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(
    Path(id): Path<String>,
    Json(mut body): Json<UpdateRole>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let found: Role = Role::find_by_id(&id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &id)))?;
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&found.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    body.validate()?;
    body.updated_by = Some(auth.id.clone());
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

async fn remove(Path(id): Path<String>, Extension(auth): Extension<Auth>) -> APIResult {
    let found: Role = Role::find_by_id(&id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &id)))?;
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&found.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    Role::delete_by_id(&id).await?;
    Ok(reply!(found))
}

async fn grant(Json(mut body): Json<UserGrantRole>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let users = User::find_by_ids(body.user_ids.clone()).await?;
    let user_ids: Vec<String> = users.iter().map(|v| v.id.clone()).collect();
    let found = body.user_ids.iter().find(|v| !user_ids.contains(&v));
    if let Some(user_id) = found {
        return Err(reject!(format!("用户 {:?} 不存在", user_id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    if !auth.is_admin && !user_roles.into_iter().any(|v| v.role_level < role.level) {
        return Err(reject!(format!("不能操作高等级角色 {:?}", role.id)));
    }
    let user_roles = UserRole::find_by_role(&body.role_id).await?;
    let user_ids: Vec<String> = user_roles.into_iter().map(|v| v.user_id.clone()).collect();
    let found = body.user_ids.iter().find(|v| user_ids.contains(&v));
    if let Some(found) = found {
        return Err(reject!(format!(
            "用户 {} 已赋予角色 {}",
            found, &body.role_id
        )));
    }
    body.role_level = role.level;
    let granted = body.save().await?;
    Ok(reply!(granted))
}

async fn revoke(Json(body): Json<UserRevokeRole>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let users = User::find_by_ids(body.user_ids.clone()).await?;
    let user_ids: Vec<String> = users.iter().map(|v| v.id.clone()).collect();
    let found = body.user_ids.iter().find(|v| !user_ids.contains(&v));
    if let Some(user_id) = found {
        return Err(reject!(format!("用户 {:?} 不存在", user_id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    if !auth.is_admin && !user_roles.into_iter().any(|v| v.role_level < role.level) {
        return Err(reject!(format!("不能操作高等级角色 {:?}", role.id)));
    }
    let user_roles = UserRole::find_by_role(&body.role_id).await?;
    let user_ids: Vec<String> = user_roles.into_iter().map(|v| v.user_id.clone()).collect();
    let found = body.user_ids.iter().find(|v| !user_ids.contains(&v));
    if let Some(found) = found {
        return Err(reject!(format!(
            "用户 {} 未赋予角色 {}",
            found, &body.role_id
        )));
    }
    let revoked = body.save().await?;
    Ok(reply!(revoked))
}

async fn change(Json(body): Json<UserChangeRole>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let users = User::find_by_ids(body.user_ids.clone()).await?;
    let user_ids: Vec<String> = users.iter().map(|v| v.id.clone()).collect();
    let found = body.user_ids.iter().find(|v| !user_ids.contains(&v));
    if let Some(user_id) = found {
        return Err(reject!(format!("用户 {:?} 不存在", user_id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    if !auth.is_admin && !user_roles.into_iter().any(|v| v.role_level < role.level) {
        return Err(reject!(format!("不能操作高等级角色 {:?}", role.id)));
    }
    let user_roles = body.save(role, users).await?;
    Ok(reply!(user_roles))
}

async fn expire(Json(body): Json<UpdateUserRole>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    if !auth.is_admin && !user_roles.into_iter().any(|v| v.role_level < role.level) {
        return Err(reject!(format!("不能操作高等级角色 {:?}", role.id)));
    }
    let user_role = body.save().await?;
    Ok(reply!(user_role))
}

pub fn apply_routes() -> Router<BoxRoute> {
    let router = Router::new();
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    router.route("/role", post(create).get(all))
        .route("/role/:id", put(update).get(one).delete(remove))
        .route("/grant/role", post(grant))
        .route("/revoke/role", post(revoke))
        .route("/change/role", post(change))
        .route("/expire/role", post(expire))
        .layer(restrict_layer)
        .boxed()
}
