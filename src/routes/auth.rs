use crate::auth::user::Claims;
use crate::db;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterInfo {
    username: String,
    password: String,
    role: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
}

#[post("/register", data = "<info>")]
pub fn register(info: Json<RegisterInfo>) -> Result<Json<User>, Status> {
    let conn = &mut db::establish_connection();
    let hashed_password =
        hash(&info.password, DEFAULT_COST).map_err(|_| Status::InternalServerError)?;

    let user_role = match info.role.as_deref() {
        Some("developer") => "developer",
        Some("project_manager") => "project_manager",
        _ => "developer",
    }
    .to_string();

    let new_user = NewUser {
        username: info.username.clone(),
        password_hash: hashed_password,
        role: user_role,
    };

    diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/login", data = "<info>")]
pub fn login(info: Json<LoginInfo>) -> Result<Json<TokenResponse>, Status> {
    let conn = &mut db::establish_connection();
    let user = users
        .filter(username.eq(&info.username))
        .first::<User>(conn)
        .map_err(|_| Status::Unauthorized)?;
    if verify(&info.password, &user.password_hash).map_err(|_| Status::InternalServerError)? {
        let claims = Claims { sub: user.id, role: user.role.clone(), exp: 10000000000 };
        let token =
            encode(&Header::default(), &claims, &EncodingKey::from_secret("SECRET".as_ref()))
                .map_err(|_| Status::InternalServerError)?;
        Ok(Json(TokenResponse { token }))
    } else {
        Err(Status::Unauthorized)
    }
}
