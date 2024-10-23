use crate::auth::ApiKey;
use crate::db;
use crate::models::snack::{NewSnack, Snack};
use crate::schema::snacks::dsl::{id, snacks};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::snacks)]
pub struct UpdateSnack {
    pub name: Option<String>,
    pub category: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub image_url: Option<String>,
}

#[post("/snack", data = "<snack_data>")]
pub fn create_snack(_api_key: ApiKey, snack_data: Json<NewSnack>) -> Result<Json<Snack>, Status> {
    let snack = snack_data.into_inner();

    let mut conn = db::establish_connection();

    diesel::insert_into(snacks)
        .values(&snack)
        .get_result(&mut conn)
        .map(Json)
        .map_err(|err| {
            println!("Database error: {:?}", err);
            Status::InternalServerError
        })
}

#[get("/snacks")]
pub fn list_snacks(_api_key: ApiKey) -> Result<Json<Vec<Snack>>, Status> {
    let mut conn = db::establish_connection();

    snacks
        .limit(10)
        .select(Snack::as_select())
        .load(&mut conn)
        .map(Json)
        .map_err(|err| {
            println!("Database error: {:?}", err);
            Status::InternalServerError
        })
}

#[patch("/snack/<snack_id>", data = "<snack_data>")]
pub fn update_snack(
    _api_key: ApiKey,
    snack_id: i32,
    snack_data: Json<UpdateSnack>,
) -> Result<Json<Snack>, Status> {
    let mut conn = db::establish_connection();

    diesel::update(snacks.find(snack_id))
        .set(&snack_data.into_inner())
        .get_result(&mut conn)
        .map(Json)
        .map_err(|err| {
            println!("Database error: {:?}", err);
            match err {
                diesel::result::Error::NotFound => Status::NotFound,
                _ => Status::InternalServerError
            }
        })
}
#[delete("/snack/<snack_id>")]
pub fn delete_snack(_api_key: ApiKey, snack_id: i32) -> Status {
    let mut conn = db::establish_connection();

    match diesel::delete(snacks.find(snack_id)).execute(&mut conn) {
        Ok(count) => {
            if count > 0 {
                Status::NoContent
            } else {
                Status::NotFound
            }
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            Status::InternalServerError
        }
    }
}
