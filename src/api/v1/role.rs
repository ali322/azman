use axum::{
    extract::{Extension, Path},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dto::{NewRole, UpdateRole, UpdateUserRole, UserChangeRole, UserGrantRole, UserRevokeRole},
        vo::{Domain, Role},
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use validator::Validate;

async fn all() -> APIResult {
    let all = Role::find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<i32>) -> APIResult {
    let one = Role::find_one(id).await?;
    Ok(reply!(one))
}

async fn create(Json(mut body): Json<NewRole>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        if Domain::find_one(&domain_id).await.is_err() {
            return Err(reject!(format!("来源域 {} 不存在", &domain_id)));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    body.validate()?;
    body.created_by = Some(auth.id.clone());
    body.domain_id = domain_id;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(
    Path(id): Path<i32>,
    Json(mut body): Json<UpdateRole>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = Role::find_one(id).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("角色 {:?} 不属于来源域", found.id)));
        }
    }
    body.validate()?;
    body.updated_by = Some(auth.id.clone());
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

async fn remove(Path(id): Path<i32>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = Role::find_one(id).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("角色 {:?} 不属于来源域", found.id)));
        }
    }
    let removed = Role::delete_one(id).await?;
    Ok(reply!(removed))
}

async fn grant(Json(body): Json<UserGrantRole>, Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        let role = Role::find_one(body.role_id).await?;
        // let user = guard!(User::find_one(body.user_id, &conn));
        if role.domain_id != auth.domain_id.unwrap() {
            return Err(reject!(format!("角色 {:?} 不属于来源域", role.id)));
        }
        if role.level < auth.role_level {
            return Err(reject!(format!("角色 {:?} 超过当前角色层级", role.id)));
        }
    }
    let granted = body.save().await?;
    Ok(reply!(granted))
}

async fn revoke(Json(body): Json<UserRevokeRole>, Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        let role = Role::find_one(body.role_id).await?;
        // let user = guard!(User::find_one(body.user_id, &conn));
        if role.domain_id != auth.domain_id.unwrap() {
            return Err(reject!(format!("角色 {:?} 不属于来源域", role.id)));
        }
        if role.level < auth.role_level {
            return Err(reject!(format!("角色 {:?} 超过当前角色层级", role.id)));
        }
    }
    let revoked = body.save().await?;
    Ok(reply!(revoked))
}

async fn change(Json(body): Json<UserChangeRole>, Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        let roles = Role::find_by_ids(body.role_ids.clone(), auth.domain_id).await?;
        for role in roles {
            if role.level < auth.role_level {
                return Err(reject!(format!("角色 {:?} 超过当前角色层级", role.id)));
            }
        }
    }
    let user_roles = body.save().await?;
    Ok(reply!(user_roles))
}

async fn expire(Json(body): Json<UpdateUserRole>, Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        let role = Role::find_one(body.role_id).await?;
        // let user = guard!(User::find_one(body.user_id, &conn));
        if role.domain_id != auth.domain_id.unwrap() {
            return Err(reject!(format!("角色 {:?} 不属于来源域", role.id)));
        }
        if role.level < auth.role_level {
            return Err(reject!(format!("角色 {:?} 超过当前角色层级", role.id)));
        }
    }
    let user_role = body.save().await?;
    Ok(reply!(user_role))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/role", post(create).get(all))
        .route("/role/:id", put(update).get(one).delete(remove))
        .route("/grant/role", post(grant))
        .route("/revoke/role", post(revoke))
        .route("/change/role", post(change))
        .route("/expire/role", post(expire))
        .layer(restrict_layer)
        .boxed()
}
