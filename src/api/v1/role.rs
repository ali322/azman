use axum::{Json, Router, routing::BoxRoute, handler::post};

use crate::{repository::{dto::NewRole}, util::APIResult};
use validator::Validate;

async fn create(Json(body): Json<NewRole>) -> APIResult {
  body.validate()?;
  let created = body.create().await?;
  Ok(reply!(created))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
  v1.route("/role", post(create)).boxed()
}