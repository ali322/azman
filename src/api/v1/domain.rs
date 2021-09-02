use axum::{
    extract::Path,
    handler::{post, put},
    routing::BoxRoute,
    Json, Router,
};
use std::env;
use uuid::Uuid;

use crate::{
    repository::{
        dto::{NewDomain, NewRole, UpdateDomain},
        vo::Domain,
    },
    util::APIResult,
};
use validator::Validate;

async fn all() -> APIResult {
    let all = Domain::find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one = Domain::find_one(id).await?;
    Ok(reply!(one))
}

async fn create(Json(body): Json<NewDomain>) -> APIResult {
    body.validate()?;
    let id = Uuid::new_v4().to_string();
    // let created = body.create().await?;
    let admin_role_name =
        env::var("ADMIN_ROLE_NAME").expect("environment variable ADMIN_ROLE_NAME must be set");
    let common_role_name =
        env::var("COMMON_ROLE_NAME").expect("environment variable COMMON_ROLE_NAME must be set");
    let new_role = NewRole {
        name: admin_role_name.clone(),
        description: None,
        value: admin_role_name.clone(),
        level: 1,
        domain_id: id.clone(),
        created_by: "None".to_string(),
        updated_by: "None".to_string(),
    };
    let admin_role = new_role.create().await?;
    let new_role = NewRole {
        name: common_role_name.clone(),
        description: None,
        value: common_role_name.clone(),
        level: 1,
        domain_id: id.clone(),
        created_by: "None".to_string(),
        updated_by: "None".to_string(),
    };
    let common_role = new_role.create().await?;
    let body = NewDomain {
        admin_role_id: admin_role.id,
        default_role_id: common_role.id,
        ..body
    };
    let created = body.create(id).await?;
    Ok(reply!(created))
}

async fn update(Path(id): Path<String>, Json(body): Json<UpdateDomain>) -> APIResult {
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/domain", post(create).get(all))
        .route("/domain/:id", put(update).get(one))
        .boxed()
}
