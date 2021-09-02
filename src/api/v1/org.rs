use axum::{
    extract::{Extension, Path},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};

use crate::{
    repository::{
        dto::{NewOrg, UpdateOrg, UserJoinOrg, UserLeaveOrg},
        vo::{Domain, Org},
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use tower_http::auth::RequireAuthorizationLayer;
use validator::Validate;

async fn all(Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        let domain_id = match auth.domain_id {
            Some(val) => val,
            None => return Err(reject!("来源域不能为空")),
        };
        if Domain::find_one(domain_id.clone()).await.is_err() {
            return Err(reject!(format!("来源域 {} 不存在", domain_id.clone())));
        }
    }
    let all = Org::find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one = Org::find_one(id).await?;
    Ok(reply!(one))
}

async fn create(Json(body): Json<NewOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        if Domain::find_one(domain_id.clone()).await.is_err() {
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
    Path(id): Path<String>,
    Json(body): Json<UpdateOrg>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = Org::find_one(id.clone()).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("组织 {:?} 不属于来源域", found.id)));
        }
    }
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

async fn join(Json(body): Json<UserJoinOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = Org::find_one(body.org_id.clone()).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("组织 {:?} 不属于来源域", found.id)));
        }
    }
    let joined = body.save().await?;
    Ok(reply!(joined))
}

async fn leave(Json(body): Json<UserLeaveOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = Org::find_one(body.org_id.clone()).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("组织 {:?} 不属于来源域", found.id)));
        }
    }
    let left = body.save().await?;
    Ok(reply!(left))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/org", post(create).get(all))
        .route("/org/:id", put(update).get(one))
        .route("/join/org", post(join))
        .route("/leave/org", post(leave))
        .layer(restrict_layer)
        .boxed()
}
