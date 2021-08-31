use axum::{
    routing::BoxRoute,
    Router,
};

mod role;

pub fn apply_routes() -> Router<BoxRoute> {
    let mut v1 = Router::new().boxed();
    v1 = role::apply_routes(v1.boxed());
    v1
}
