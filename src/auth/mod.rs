pub struct ApiKey(pub String);
use rocket::{http::Status, request::{FromRequest, Outcome}, Request};
use std::env;

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = request.headers().get_one("x-api-key");

        match api_key {
            None => Outcome::Error((Status::Unauthorized, ApiKeyError::Missing)),
            Some(api_key) => {
                if api_key == env::var("AUTH_API_KEY").unwrap() {
                    Outcome::Success(ApiKey(api_key.to_string()))
                } else {
                    Outcome::Error((Status::Unauthorized, ApiKeyError::Invalid))
                }
            }
        }
    }
}