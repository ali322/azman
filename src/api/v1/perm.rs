use axum::{
    extract::{Extension, Path, Query},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dao::{Domain, Perm, Role, RolePerm, UserRole},
        dto::{NewPerm, QueryPerm, RoleGrantPerm, RoleRevokePerm, UpdatePerm},
        Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use validator::Validate;

async fn all(Query(q): Query<QueryPerm>, Extension(_): Extension<Auth>) -> APIResult {
    // let all = Perm::find_all(domain_id).await?;
    let all = q.find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one: Perm = Perm::find_by_id(&id).await?;
    Ok(reply!(one))
}

async fn create(Json(mut body): Json<NewPerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain = match Domain::find_by_id(&body.domain_id).await {
        Ok(val) => val,
        Err(_) => return Err(reject!(format!("来源域 {} 不存在", &body.domain_id))),
    };
    let user_roles = UserRole::find_by_user(&auth.id, Some(&body.domain_id)).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    body.validate()?;
    body.created_by = Some(auth.id);
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(
    Path(id): Path<String>,
    Json(mut body): Json<UpdatePerm>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let found = Perm::find_by_id(&id)
        .await
        .map_err(|_| reject!(format!("权限 {} 不存在", &id)))?;
    let user_roles = UserRole::find_by_user(&auth.id, Some(&found.domain_id)).await?;
    let domain = Domain::find_by_id(&found.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    body.validate()?;
    body.updated_by = Some(auth.id);
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

async fn remove(Path(id): Path<String>, Extension(auth): Extension<Auth>) -> APIResult {
    let found = Perm::find_by_id(&id)
        .await
        .map_err(|_| reject!(format!("权限 {} 不存在", &id)))?;
    let user_roles = UserRole::find_by_user(&auth.id, Some(&found.domain_id)).await?;
    let domain = Domain::find_by_id(&found.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    Perm::delete_by_id(&id).await?;
    Ok(reply!(found))
}

async fn grant(Json(body): Json<RoleGrantPerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let perm = Perm::find_by_id(&body.perm_id)
        .await
        .map_err(|_| reject!(format!("权限 {} 不存在", &body.perm_id)))?;
    if role.domain_id != perm.domain_id {
        return Err(reject!("角色和权限不属于同一个域"));
    }
    let user_roles = UserRole::find_by_user(&auth.id, Some(&role.domain_id)).await?;
    let domain = Domain::find_by_id(&role.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    if RolePerm::find_by_id(&body.role_id, &body.perm_id)
        .await
        .is_ok()
    {
        return Err(reject!(format!(
            "角色 {} 已赋予权限 {}",
            body.role_id, body.perm_id
        )));
    }
    let granted = body.save().await?;
    Ok(reply!(granted))
}

async fn revoke(Json(body): Json<RoleRevokePerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let perm = Perm::find_by_id(&body.perm_id)
        .await
        .map_err(|_| reject!(format!("权限 {} 不存在", &body.perm_id)))?;
    if role.domain_id != perm.domain_id {
        return Err(reject!("角色和权限不属于同一个域"));
    }
    let user_roles = UserRole::find_by_user(&auth.id, Some(&role.domain_id)).await?;
    let domain = Domain::find_by_id(&role.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    if RolePerm::find_by_id(&body.role_id, &body.perm_id)
        .await
        .is_err()
    {
        return Err(reject!(format!(
            "角色 {} 未赋予权限 {}",
            body.role_id, body.perm_id
        )));
    }
    let revoked = body.save().await?;
    Ok(reply!(revoked))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/perm", post(create).get(all))
        .route("/perm/:id", put(update).get(one).delete(remove))
        .route("/grant/perm", post(grant))
        .route("/revoke/perm", post(revoke))
        .layer(restrict_layer)
        .boxed()
}
