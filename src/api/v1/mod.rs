use crate::util::Cors;
use axum::{routing::BoxRoute, Router};
use tower::layer::layer_fn;

mod auth;
mod domain;
mod org;
mod perm;
mod rbac;
mod role;
mod user;

pub fn apply_routes() -> Router<BoxRoute> {
    auth::apply_routes()
        .or(user::apply_routes())
        .or(domain::apply_routes())
        .or(org::apply_routes())
        .or(role::apply_routes())
        .or(perm::apply_routes())
        .or(rbac::apply_routes())
        .layer(layer_fn(|inner| Cors { inner }))
        .boxed()
}
