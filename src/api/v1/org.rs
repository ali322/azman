use axum::{
    extract::Path,
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};

use crate::{
    repository::{
        dto::{NewOrg, UpdateOrg, UserJoinOrg, UserLeaveOrg},
        vo::Org,
    },
    util::APIResult,
};
use validator::Validate;

async fn all() -> APIResult {
    let all = Org::find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one = Org::find_one(id).await?;
    Ok(reply!(one))
}

async fn create(Json(body): Json<NewOrg>) -> APIResult {
    body.validate()?;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(Path(id): Path<String>, Json(body): Json<UpdateOrg>) -> APIResult {
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

async fn join(Json(body): Json<UserJoinOrg>) -> APIResult {
    let joined = body.save().await?;
    Ok(reply!(joined))
}

async fn leave(Json(body): Json<UserLeaveOrg>) -> APIResult {
    let left = body.save().await?;
    Ok(reply!(left))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/org", post(create).get(all))
        .route("/org/:id", put(update).get(one))
        .route("/join/org", post(join))
        .route("/leave/org", post(leave))
        .boxed()
}
