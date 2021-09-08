use axum::{extract::Query, handler::post, routing::BoxRoute, Json, Router};
use std::collections::HashMap;
use validator::Validate;

use crate::{
    repository::{
        dao::{Domain, Org, Role, User, UserOrg, UserRole},
        dto::{ConnectUser, LoginUser, NewUser, UserGrantRole},
        Dao,
    },
    util::{
        jwt::{self, Auth},
        APIResult,
    },
};

async fn register(
    Query(query): Query<HashMap<String, String>>,
    Json(body): Json<NewUser>,
) -> APIResult {
    body.validate()?;
    if User::find_by_username(&body.username).await.is_ok() {
        return Err(reject!("用户已存在"));
    }
    let domain = match query.get("from") {
        Some(domain_id) => {
            let domain = match Domain::find_by_id(&domain_id).await {
                Ok(val) => val,
                Err(_) => return Err(reject!(format!("来源域 {} 不存在", domain_id))),
            };
            domain
        }
        None => return Err(reject!("来源域不能为空")),
    };
    let role: Role = match domain.default_role_id.clone() {
        Some(role_id) => {
            let role = Role::find_by_id(&role_id).await?;
            role
        }
        None => {
            return Err(reject!(format!(
                "来源域 {} 没有默认角色",
                domain.id.clone()
            )))
        }
    };
    let user = body.create().await?;
    let user_grant_role = UserGrantRole {
        user_id: user.id.clone(),
        role_id: role.id.clone(),
    };
    user_grant_role.save().await?;
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        domain_id: Some(domain.id.clone()),
        org_id: vec![],
        role_id: vec![role.id.clone()],
        role_level: role.level,
        is_admin: false,
    });
    Ok(reply!({
      "token": token, "user": user,"domain": domain, "roles": vec![role], "orgs": []
    }))
}

async fn login(
    Query(query): Query<HashMap<String, String>>,
    Json(body): Json<LoginUser>,
) -> APIResult {
    body.validate()?;
    let user_dao = match User::find_by_username_or_email(&body.username_or_email).await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user_dao.password) {
        return Err(reject!("密码不正确"));
    }
    if user_dao.is_actived == 0 {
        return Err(reject!("用户被禁用"));
    }

    let user = body.login(&user_dao).await?;
    let mut roles: Vec<Role> = vec![];
    let mut role_ids: Vec<String> = vec![];
    let mut orgs: Vec<Org> = vec![];
    let mut org_ids: Vec<String> = vec![];
    let mut domain: Option<Domain> = None;
    let domain_id = query.get("from").clone().map(|v| v.clone());
    let is_admin = user.sys_role.clone().unwrap() == "admin";
    if !is_admin {
        domain = match domain_id.clone() {
            Some(v) => {
                let domain = match Domain::find_by_id(&v).await {
                    Ok(val) => val,
                    Err(_) => return Err(reject!(format!("来源域 {} 不存在", v.clone()))),
                };
                Some(domain)
            }
            None => return Err(reject!("来源域不能为空")),
        };
        let user_orgs = UserOrg::find_by_user(&user.id).await?;
        org_ids = user_orgs.iter().map(|v| v.org_id.clone()).collect();
        orgs = Org::find_by_ids(org_ids.clone(), domain_id.clone()).await?;
        let user_roles = UserRole::find_by_user(&user.id).await?;
        role_ids = user_roles.iter().map(|v| v.role_id.clone()).collect();
        roles = Role::find_by_ids(role_ids.clone(), domain_id.clone()).await?;
        roles.sort_by(|a, b| a.level.cmp(&b.level));
    }
    let role_level = if roles.len() > 0 { roles[0].level } else { 999 };
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        domain_id,
        org_id: org_ids,
        role_id: role_ids,
        role_level,
        is_admin,
    });
    Ok(reply!({
      "token": token, "user": user, "domain": domain, "roles": roles, "orgs": orgs
    }))
}

async fn connect(
    Query(query): Query<HashMap<String, String>>,
    Json(body): Json<ConnectUser>,
) -> APIResult {
    body.validate()?;
    let domain = match query.get("from") {
        Some(domain_id) => {
            let domain = match Domain::find_by_id(&domain_id).await {
                Ok(val) => val,
                Err(_) => return Err(reject!(format!("来源域 {} 不存在", domain_id))),
            };
            domain
        }
        None => return Err(reject!("来源域不能为空")),
    };
    let user: User;
    let mut roles: Vec<Role>;
    let role_ids: Vec<String>;
    let role_level: i32;
    let orgs: Vec<Org>;
    let org_ids: Vec<String>;
    if let Ok(val) = User::find_by_username(&body.username).await {
        user = val.into();
        let user_orgs = UserOrg::find_by_user(&user.id).await?;
        org_ids = user_orgs.iter().map(|v| v.org_id.clone()).collect();
        orgs = Org::find_by_ids(org_ids.clone(), Some(domain.id.clone())).await?;

        let user_roles = UserRole::find_by_user(&user.id).await?;
        role_ids = user_roles.iter().map(|v| v.role_id.clone()).collect();
        roles = Role::find_by_ids(role_ids.clone(), Some(domain.id.clone())).await?;
        roles.sort_by(|a, b| a.level.cmp(&b.level));
        role_level = if roles.len() > 0 { roles[0].level } else { 999 };
    } else {
        user = body.create().await?;
        let role: Role = match domain.default_role_id.clone() {
            Some(role_id) => {
                let role = Role::find_by_id(role_id).await?;
                role
            }
            None => {
                return Err(reject!(format!(
                    "来源域 {} 没有默认角色",
                    domain.id.clone()
                )))
            }
        };
        roles = vec![role.clone()];
        role_ids = vec![role.id.clone()];
        role_level = role.level;
        org_ids = vec![];
        orgs = vec![];
        let user_grant_role = UserGrantRole {
            user_id: user.id.clone(),
            role_id: role.id.clone(),
        };
        user_grant_role.save().await?;
    };
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        domain_id: Some(domain.id.clone()),
        role_id: role_ids,
        org_id: org_ids,
        role_level,
        is_admin: false,
    });
    Ok(reply!({
      "token": token, "user": user,"domain": domain, "roles": roles, "orgs": orgs
    }))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/register", post(register))
        .route("/login", post(login))
        .route("/connect", post(connect))
        .boxed()
}
