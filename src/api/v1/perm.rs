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
        dto::{
            BatchInsertPerm, NewPerm, QueryPerm, RoleChangePerm, RoleGrantPerm, RoleRevokePerm,
            UpdatePerm,
        },
        Dao,
    },
    util::{jwt::Auth, now, restrict::Restrict, uuid_v4, APIResult},
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
        Err(_) => {
            return Err(reject!(format!(
                "来源域 {:?} 不存在",
                body.domain_id.clone()
            )))
        }
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
    body.updated_by = Some(auth.id);
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

async fn remove(Path(id): Path<String>, Extension(auth): Extension<Auth>) -> APIResult {
    let found = Perm::find_by_id(&id)
        .await
        .map_err(|_| reject!(format!("权限 {} 不存在", &id)))?;
    let user_roles = UserRole::find_by_user(&auth.id).await?;
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
    let perms = Perm::find_by_ids(body.perm_ids.clone(), None).await?;
    let perm_ids: Vec<String> = perms.iter().map(|v| v.id.clone()).collect();
    let found = body.perm_ids.iter().find(|v| !perm_ids.contains(&v));
    if let Some(perm_id) = found {
        return Err(reject!(format!("权限 {:?} 不存在", perm_id)));
    }
    let found = perms.iter().find(|v| v.domain_id != role.domain_id);
    if let Some(perm) = found {
        return Err(reject!(format!("权限 {:?} 和角色不属于同一个域", perm.id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&role.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    let role_perms = RolePerm::find_by_role(&body.role_id).await?;
    let perm_ids: Vec<String> = role_perms.into_iter().map(|v| v.perm_id.clone()).collect();
    let found = body.perm_ids.iter().find(|v| perm_ids.contains(&v));
    if let Some(found) = found {
        return Err(reject!(format!(
            "角色 {} 已赋予权限 {}",
            &body.role_id, found
        )));
    }
    let granted = body.save().await?;
    Ok(reply!(granted))
}

async fn revoke(Json(body): Json<RoleRevokePerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let perms = Perm::find_by_ids(body.perm_ids.clone(), None).await?;
    let perm_ids: Vec<String> = perms.iter().map(|v| v.id.clone()).collect();
    let found = body.perm_ids.iter().find(|v| !perm_ids.contains(&v));
    if let Some(perm_id) = found {
        return Err(reject!(format!("权限 {:?} 不存在", perm_id)));
    }
    let found = perms.iter().find(|v| v.domain_id != role.domain_id);
    if let Some(perm) = found {
        return Err(reject!(format!("权限 {:?} 和角色不属于同一个域", perm.id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&role.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    let role_perms = RolePerm::find_by_role(&body.role_id).await?;
    let perm_ids: Vec<String> = role_perms.into_iter().map(|v| v.perm_id.clone()).collect();
    let found = body.perm_ids.iter().find(|v| !perm_ids.contains(&v));
    if let Some(found) = found {
        return Err(reject!(format!(
            "角色 {} 未赋予权限 {}",
            &body.role_id, found
        )));
    }
    let revoked = body.save().await?;
    Ok(reply!(revoked))
}

async fn change(Json(body): Json<RoleChangePerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let role: Role = Role::find_by_id(&body.role_id)
        .await
        .map_err(|_| reject!(format!("角色 {} 不存在", &body.role_id)))?;
    let perms = Perm::find_by_ids(body.perm_ids.clone(), None).await?;
    let perm_ids: Vec<String> = perms.iter().map(|v| v.id.clone()).collect();
    let found = body.perm_ids.iter().find(|v| !perm_ids.contains(&v));
    if let Some(perm_id) = found {
        return Err(reject!(format!("权限 {:?} 不存在", perm_id)));
    }
    let found = perms.iter().find(|v| v.domain_id != role.domain_id);
    if let Some(perm) = found {
        return Err(reject!(format!("权限 {:?} 和角色不属于同一个域", perm.id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&role.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    let role_perms = body.save().await?;
    Ok(reply!(role_perms))
}

async fn create_all(
    Json(body): Json<BatchInsertPerm>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&body.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    let perms: Vec<Perm> = body
        .perms
        .iter()
        .map(|v| {
            let id = uuid_v4();
            return Perm {
                id,
                name: v.name.clone(),
                description: Some(v.name.clone()),
                value: v.value.clone(),
                domain_id: body.domain_id.clone(),
                is_deleted: 0,
                created_at: now(),
                updated_at: now(),
                created_by: Some(auth.id.clone()),
                updated_by: Some(auth.id.clone()),
            };
        })
        .collect();
    Perm::create_all(&perms).await?;
    Ok(reply!(perms))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/perm", post(create).get(all))
        .route("/perm/:id", put(update).get(one).delete(remove))
        .route("/grant/perm", post(grant))
        .route("/revoke/perm", post(revoke))
        .route("/change/perm", post(change))
        .route("/batch/perm", post(create_all))
        .layer(restrict_layer)
        .boxed()
}
