use axum::{
    extract::{Extension, Path},
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;

use crate::{
    repository::{
        dao::{domain, Domain},
        dto::{NewDomain, UpdateDomain},
        Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use validator::Validate;

async fn all(Extension(_auth): Extension<Auth>) -> APIResult {
    let all = Domain::find_all(None).await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    use domain::IntoVo;
    let one = Domain::find_by_id(&id).await?.into_vo().await?;
    Ok(reply!(one))
}

async fn create(Json(body): Json<NewDomain>, Extension(auth): Extension<Auth>) -> APIResult {
    if !auth.is_admin {
        return Err(reject!("仅管理员可访问"));
    }
    body.validate()?;
    let created = body.create(&auth.id).await?;
    Ok(reply!(created))
}

async fn update(
    Path(id): Path<String>,
    Json(body): Json<UpdateDomain>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    if !auth.is_admin {
        return Err(reject!("仅管理员可访问"));
    }
    body.validate()?;
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/domain", post(create).get(all))
        .route("/domain/:id", put(update).get(one))
        .layer(restrict_layer)
        .boxed()
}
