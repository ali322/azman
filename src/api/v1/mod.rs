use axum::{handler::get, routing::BoxRoute, Router};

async fn index() -> &'static str {
    "hello world"
}

pub fn apply_routes() -> Router<BoxRoute> {
    let v1 = Router::new().route("/", get(index));
    v1.boxed()
}
