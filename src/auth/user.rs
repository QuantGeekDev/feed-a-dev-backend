use crate::models::user::User;
use diesel::prelude::*;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

pub struct AuthenticatedUser(pub User);

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: i32,
    pub(crate) role: String,
    pub(crate) exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        match token {
            Some(token) if token.starts_with("Bearer ") => {
                let token = &token[7..];
                let decoding_key = DecodingKey::from_secret("your_secret_key".as_ref());
                match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)) {
                    Ok(token_data) => {
                        let mut conn = crate::db::establish_connection();
                        match crate::schema::users::dsl::users
                            .find(token_data.claims.sub)
                            .first::<User>(&mut conn)
                        {
                            Ok(user) => Outcome::Success(AuthenticatedUser(user)),
                            Err(_) => Outcome::Error((Status::Unauthorized, ())),
                        }
                    }
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
