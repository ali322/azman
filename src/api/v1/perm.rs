use axum::{
    extract::{Extension, Path},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dao::{Domain, Perm, Role, RolePerm},
        dto::{NewPerm, RoleGrantPerm, RoleRevokePerm, UpdatePerm},
        Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use validator::Validate;

async fn all(Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        if auth.domain_id.is_none() {
            return Err(reject!("来源域不能为空"));
        }
        let domain_id = auth.domain_id.clone().unwrap();
        if Domain::find_by_id(&domain_id).await.is_err() {
            return Err(reject!(format!("来源域 {} 不存在", &domain_id)));
        }
    }
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let all = Perm::find_all(domain_id).await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<i32>) -> APIResult {
    let one: Perm = Perm::find_by_id(id).await?;
    Ok(reply!(one))
}

async fn create(Json(mut body): Json<NewPerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        if Domain::find_by_id(&domain_id).await.is_err() {
            return Err(reject!(format!("来源域 {} 不存在", domain_id.clone())));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    body.validate()?;
    body.created_by = Some(auth.id);
    body.domain_id = domain_id;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(
    Path(id): Path<i32>,
    Json(mut body): Json<UpdatePerm>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found: Perm = Perm::find_by_id(id).await?.into();
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("权限 {:?} 不属于来源域", found.id)));
        }
    }
    body.validate()?;
    body.updated_by = Some(auth.id);
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

async fn grant(Json(body): Json<RoleGrantPerm>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        let role: Role = Role::find_by_id(body.role_id).await?;
        let perm: Perm = Perm::find_by_id(body.perm_id).await?;
        if role.domain_id != domain_id {
            return Err(reject!(format!("角色 {:?} 不属于来源域", role.id)));
        } else if perm.domain_id != domain_id {
            return Err(reject!(format!("行为 {:?} 不属于来源域", perm.id)));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    if RolePerm::find_by_id(body.role_id, body.perm_id)
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
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        let role: Role = Role::find_by_id(body.role_id).await?;
        let perm: Perm = Perm::find_by_id(body.perm_id).await?;
        if role.domain_id != domain_id {
            return Err(reject!(format!("角色 {:?} 不属于来源域", role.id)));
        } else if perm.domain_id != domain_id {
            return Err(reject!(format!("行为 {:?} 不属于来源域", perm.id)));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    if RolePerm::find_by_id(body.role_id, body.perm_id)
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
        .route("/perm/:id", put(update).get(one))
        .route("/grant/perm", post(grant))
        .route("/revoke/perm", post(revoke))
        .layer(restrict_layer)
        .boxed()
}
