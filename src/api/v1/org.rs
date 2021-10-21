use axum::{
    extract::{Extension, Path, Query},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};

use crate::{
    repository::{
        dao::{Domain, Org, User, UserOrg, UserRole},
        dto::{NewOrg, QueryOrg, UpdateOrg, UserJoinOrg, UserLeaveOrg},
        Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use tower_http::auth::RequireAuthorizationLayer;
use validator::Validate;

async fn all(Query(q): Query<QueryOrg>, Extension(_): Extension<Auth>) -> APIResult {
    let all = q.find_all().await?;
    // let all: Vec<Org> = Org::find_all(domain_id).await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one: Org = Org::find_by_id(&id).await?;
    Ok(reply!(one))
}

async fn create(Json(mut body): Json<NewOrg>, Extension(auth): Extension<Auth>) -> APIResult {
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
    body.created_by = Some(auth.id);
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(
    Path(id): Path<String>,
    Json(body): Json<UpdateOrg>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    let found = Org::find_by_id(&id)
        .await
        .map_err(|_| reject!(format!("组织 {} 不存在", &id)))?;
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
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

async fn join(Json(body): Json<UserJoinOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let org: Org = Org::find_by_id(&body.org_id)
        .await
        .map_err(|_| reject!(format!("组织 {} 不存在", &body.org_id)))?;
    let users = User::find_by_ids(body.user_ids.clone()).await?;
    let user_ids: Vec<String> = users.iter().map(|v| v.id.clone()).collect();
    let found = body.user_ids.iter().find(|v| !user_ids.contains(&v));
    if let Some(user_id) = found {
        return Err(reject!(format!("用户 {:?} 不存在", user_id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&org.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    let user_orgs = UserOrg::find_by_org(&body.org_id).await?;
    let user_ids: Vec<String> = user_orgs.into_iter().map(|v| v.user_id.clone()).collect();
    let found = body.user_ids.iter().find(|v| user_ids.contains(&v));
    if let Some(found) = found {
        return Err(reject!(format!(
            "用户 {} 已参与组织 {}",
            found, &body.org_id
        )));
    }
    let joined = body.save().await?;
    Ok(reply!(joined))
}

async fn leave(Json(body): Json<UserLeaveOrg>, Extension(auth): Extension<Auth>) -> APIResult {
    let org: Org = Org::find_by_id(&body.org_id)
        .await
        .map_err(|_| reject!(format!("组织 {} 不存在", &body.org_id)))?;
    let users = User::find_by_ids(body.user_ids.clone()).await?;
    let user_ids: Vec<String> = users.iter().map(|v| v.id.clone()).collect();
    let found = body.user_ids.iter().find(|v| !user_ids.contains(&v));
    if let Some(user_id) = found {
        return Err(reject!(format!("用户 {:?} 不存在", user_id)));
    }
    let user_roles = UserRole::find_by_user(&auth.id).await?;
    let domain = Domain::find_by_id(&org.domain_id).await?;
    if !auth.is_admin
        && !user_roles
            .into_iter()
            .any(|v| v.role_id == domain.admin_role_id)
    {
        return Err(reject!(format!("仅域管理员可操作")));
    }
    let user_orgs = UserOrg::find_by_org(&body.org_id).await?;
    let user_ids: Vec<String> = user_orgs.into_iter().map(|v| v.user_id.clone()).collect();
    let found = body.user_ids.iter().find(|v| !user_ids.contains(&v));
    if let Some(found) = found {
        return Err(reject!(format!(
            "用户 {} 未参与组织 {}",
            found, &body.org_id
        )));
    }
    let left = body.save().await?;
    Ok(reply!(left))
}

pub fn apply_routes() -> Router<BoxRoute> {
    let router = Router::new();
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    router.route("/org", post(create).get(all))
        .route("/org/:id", put(update).get(one))
        .route("/join/org", post(join))
        .route("/leave/org", post(leave))
        .layer(restrict_layer)
        .boxed()
}
