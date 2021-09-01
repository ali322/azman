use axum::{
    extract::Path,
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};

use crate::{
    repository::{
        dto::{NewDomain, UpdateDomain},
        vo::Domain,
    },
    util::APIResult,
};
use validator::Validate;

async fn all() -> APIResult {
    let all = Domain::find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<i32>) -> APIResult {
    let one = Domain::find_one(id).await?;
    Ok(reply!(one))
}

async fn create(Json(body): Json<NewDomain>) -> APIResult {
    body.validate()?;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(Path(id): Path<i32>, Json(body): Json<UpdateDomain>) -> APIResult {
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/domain", post(create).get(all))
        .route("/domain/:id", put(update).get(one))
        .boxed()
}
