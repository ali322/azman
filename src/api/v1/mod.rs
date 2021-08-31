use axum::{
    routing::BoxRoute,
    Router,
};

mod role;
mod perm;
mod user;
mod auth;

pub fn apply_routes() -> Router<BoxRoute> {
    let mut v1 = Router::new().boxed();
    v1 = auth::apply_routes(v1.boxed());
    v1 = user::apply_routes(v1.boxed());
    v1 = role::apply_routes(v1.boxed());
    v1 = perm::apply_routes(v1.boxed());
    v1
}
