use axum::{
    extract::{Extension, Path},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dto::{NewPerm, RoleGrantPerm, RoleRevokePerm, UpdatePerm},
        vo::{Domain, Perm, Role},
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use validator::Validate;

async fn all() -> APIResult {
    let all = Perm::find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<i32>) -> APIResult {
    let one = Perm::find_one(id).await?;
    Ok(reply!(one))
}

async fn create(Json(body): Json<NewPerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        if Domain::find_one(&domain_id).await.is_err() {
            return Err(reject!(format!("来源域 {} 不存在", domain_id.clone())));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    body.validate()?;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(
    Path(id): Path<i32>,
    Json(body): Json<UpdatePerm>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = Perm::find_one(id).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("权限 {:?} 不属于来源域", found.id)));
        }
    }
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

async fn grant(Json(body): Json<RoleGrantPerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        let role = Role::find_one(body.role_id).await?;
        let perm = Perm::find_one(body.perm_id).await?;
        if role.domain_id != domain_id {
            return Err(reject!(format!("角色 {:?} 不属于来源域", role.id)));
        } else if perm.domain_id != domain_id {
            return Err(reject!(format!("行为 {:?} 不属于来源域", perm.id)));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    let granted = body.save().await?;
    Ok(reply!(granted))
}

async fn revoke(Json(body): Json<RoleRevokePerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        let role = Role::find_one(body.role_id).await?;
        let perm = Perm::find_one(body.perm_id).await?;
        if role.domain_id != domain_id {
            return Err(reject!(format!("角色 {:?} 不属于来源域", role.id)));
        } else if perm.domain_id != domain_id {
            return Err(reject!(format!("行为 {:?} 不属于来源域", perm.id)));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    let revoked = body.save().await?;
    Ok(reply!(revoked))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/perm", post(create).get(all))
        .route("/perm/:id", put(update).get(one))
        .route("/grant/perm", post(grant))
        .route("/revoke/perm", post(revoke))
        .layer(restrict_layer)
        .boxed()
}
