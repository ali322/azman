use axum::{Router, handler::get, routing::BoxRoute};

pub mod v1;

async fn index() -> &'static str {
  "hello world"
}

pub fn apply_routes() -> Router<BoxRoute> {
  let prefix = "/api/v1";
  let router = Router::new().route("/", get(index));
  router.nest(prefix, v1::apply_routes()).boxed()
}