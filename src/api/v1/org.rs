use axum::{
    extract::{Extension, Path},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};

use crate::{
    repository::{
        Dao,
        dao::{DomainDao, OrgDao, UserOrgDao},
        dto::{NewOrg, UpdateOrg, UserJoinOrg, UserLeaveOrg},
        vo::Org,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use tower_http::auth::RequireAuthorizationLayer;
use validator::Validate;

async fn all(Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        if auth.domain_id.is_none() {
            return Err(reject!("来源域不能为空"));
        }
        let domain_id = auth.domain_id.clone().unwrap();
        if DomainDao::find_by_id(&domain_id).await.is_err() {
            return Err(reject!(format!("来源域 {} 不存在", &domain_id)));
        }
    }
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let all: Vec<Org> = OrgDao::find_all(domain_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one: Org = OrgDao::find_by_id(&id).await?.into();
    Ok(reply!(one))
}

async fn create(Json(mut body): Json<NewOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    if !auth.is_admin {
        if DomainDao::find_by_id(&domain_id).await.is_err() {
            return Err(reject!(format!("来源域 {} 不存在", &domain_id)));
        }
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
    }
    body.validate()?;
    body.domain_id = domain_id;
    body.created_by = Some(auth.id);
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
    let found = OrgDao::find_by_id(&id).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("组织 {:?} 不属于来源域", found.id)));
        }
    }
    body.validate()?;
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

async fn join(Json(body): Json<UserJoinOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = OrgDao::find_by_id(&body.org_id).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("组织 {:?} 不属于来源域", found.id)));
        }
    }
    if UserOrgDao::find_by_id(&body.user_id, &body.org_id)
        .await
        .is_ok()
    {
        return Err(reject!(format!(
            "用户 {} 已加入组织 {}",
            &body.user_id, &body.org_id
        )));
    }
    let joined = body.save().await?;
    Ok(reply!(joined))
}

async fn leave(Json(body): Json<UserLeaveOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let domain_id = match auth.domain_id {
        Some(val) => val,
        None => return Err(reject!("来源域不能为空")),
    };
    let found = OrgDao::find_by_id(&body.org_id).await?;
    if !auth.is_admin {
        if auth.role_level > 1 {
            return Err(reject!(format!("仅域管理员可操作")));
        }
        if found.domain_id != domain_id {
            return Err(reject!(format!("组织 {:?} 不属于来源域", found.id)));
        }
    }
    if UserOrgDao::find_by_id(&body.user_id, &body.org_id)
        .await
        .is_err()
    {
        return Err(reject!(format!(
            "用户 {} 未加入组织 {}",
            &body.user_id, &body.org_id
        )));
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
