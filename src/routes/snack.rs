use crate::auth::user::AuthenticatedUser;
use crate::db;
use crate::models::snack::{CreateSnackRequest, Snack};
use crate::schema::snacks::dsl::snacks;
use crate::schema::snacks::user_id;
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
pub fn create_snack(snack_data: Json<CreateSnackRequest>, user: AuthenticatedUser) -> Result<Json<Snack>, Status> {
    let mut conn = db::establish_connection();
    let snack = snack_data.into_inner().into_new_snack(user.0.id);


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
pub fn list_snacks(user: AuthenticatedUser) -> Result<Json<Vec<Snack>>, Status> {
    let mut conn = db::establish_connection();

    let results = if user.0.role == "admin" {
        snacks
            .limit(100)
            .select(Snack::as_select())
            .load(&mut conn)
    } else {
        snacks
            .filter(user_id.eq(user.0.id))
            .limit(100)
            .select(Snack::as_select())
            .load(&mut conn)
    };

    results
        .map(Json)
        .map_err(|err| {
            println!("Database error: {:?}", err);
            Status::InternalServerError
        })
}
#[patch("/snack/<snack_id>", data = "<snack_data>")]
pub fn update_snack(
    snack_id: i32,
    snack_data: Json<UpdateSnack>,
    user: AuthenticatedUser,
) -> Result<Json<Snack>, Status> {
    let mut conn = db::establish_connection();

    let snack = snacks
        .find(snack_id)
        .first::<Snack>(&mut conn)
        .map_err(|err| {
            println!("Database error: {:?}", err);
            match err {
                diesel::result::Error::NotFound => Status::NotFound,
                _ => Status::InternalServerError
            }
        })?;

    if snack.user_id != user.0.id && user.0.role != "admin" {
        return Err(Status::Forbidden);
    }

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
pub fn delete_snack(snack_id: i32, user: AuthenticatedUser) -> Status {
    let mut conn = db::establish_connection();

    match snacks
        .find(snack_id)
        .first::<Snack>(&mut conn) {
        Ok(snack) => {
            if snack.user_id != user.0.id && user.0.role != "admin" {
                return Status::Forbidden;
            }

            match diesel::delete(snacks.find(snack_id)).execute(&mut conn) {
                Ok(_) => Status::NoContent,
                Err(err) => {
                    println!("Database error: {:?}", err);
                    Status::InternalServerError
                }
            }
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            match err {
                diesel::result::Error::NotFound => Status::NotFound,
                _ => Status::InternalServerError
            }
        }
    }
}