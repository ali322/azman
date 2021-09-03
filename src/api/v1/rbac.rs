use std::collections::HashMap;

use axum::{extract::Extension, handler::post, routing::BoxRoute, Json, Router};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dto::Access,
        vo::{Perm, RolePerm, UserRole},
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};

async fn access(Json(body): Json<Access>, Extension(auth): Extension<Auth>) -> APIResult {
    let role_perms = RolePerm::find_by_role(body.role_id).await?;
    let mut permMap: HashMap<String, bool> = HashMap::new();
    let domain_id = if auth.is_admin { None } else { auth.domain_id };
    let perms = Perm::find_by_ids(body.perm_id, domain_id).await?;
    for perm in perms.into_iter() {
        permMap.insert(
            perm.name.clone(),
            role_perms.iter().any(|v| v.perm_id == perm.id.unwrap()),
        );
    }
    let user_role = UserRole::find_one(auth.id, body.role_id).await;
    if user_role.is_err() {
        for (_, item) in permMap.iter_mut() {
            *item = false
        }
    }
    Ok(reply!(permMap))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/access", post(access))
        .layer(restrict_layer)
        .boxed()
}
