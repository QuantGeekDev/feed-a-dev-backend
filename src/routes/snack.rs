use crate::auth::ApiKey;
use crate::db;
use crate::models::snack::{NewSnack, Snack};
use crate::schema::snacks;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/snack", data = "<snack_data>")]
pub fn create_snack(_api_key: ApiKey, snack_data: Json<NewSnack>) -> Result<Json<Snack>, Status> {
    let snack = snack_data.into_inner();

    let mut conn = db::establish_connection();

    diesel::insert_into(snacks::table)
        .values(&snack)
        .get_result(&mut conn)
        .map(Json)
        .map_err(|err| {
            println!("Database error: {:?}", err);
            Status::InternalServerError
        })
}