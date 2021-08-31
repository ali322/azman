use axum::{
    extract::Path,
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};

use crate::{
    repository::{
        dto::{NewRole, UpdateRole},
        vo::Role,
    },
    util::APIResult,
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

async fn create(Json(body): Json<NewRole>) -> APIResult {
    body.validate()?;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(Path(id): Path<i32>, Json(body): Json<UpdateRole>) -> APIResult {
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

async fn remove(Path(id): Path<i32>) -> APIResult {
    let removed = Role::delete_one(id).await?;
    Ok(reply!(removed))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/role", post(create).get(all))
        .route("/role/:id", put(update).get(one).delete(remove))
        .boxed()
}
