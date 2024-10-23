use rocket::catch;
use rocket::serde::json::Json;
use rocket::Request;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    message: String,
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 401,
        message: format!("Unauthorized access to {}", req.uri()),
    })
}