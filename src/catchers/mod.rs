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

#[catch(404)]
pub fn not_found(_req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 404,
        message: "Page not found".to_string(),
    })
}

#[catch(500)]
pub fn internal_server_error(_req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 500,
        message: "Internal Server Error".to_string(),
    })
}
