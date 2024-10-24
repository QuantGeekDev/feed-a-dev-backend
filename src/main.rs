extern crate dotenv;
#[macro_use]
extern crate rocket;
mod routes;
mod models;
mod auth;
mod catchers;
mod db;
mod schema;

use crate::routes::auth::{login, register};
use crate::routes::relationship::{invite_project_manager, list_developers, list_snacks, respond_to_invite};
use crate::routes::snack::{create_snack, delete_snack, update_snack};
use dotenv::dotenv;
use rocket::*;

#[get("/")]
fn index() -> &'static str {
    " _______  _______  _______  ______          _______         ______   _______  __   __
    |       ||       ||       ||      |        |   _   |       |      | |       ||  | |  |
    |    ___||    ___||    ___||  _    | ____  |  |_|  | ____  |  _    ||    ___||  |_|  |
    |   |___ |   |___ |   |___ | | |   ||____| |       ||____| | | |   ||   |___ |       |
    |    ___||    ___||    ___|| |_|   |       |       |       | |_|   ||    ___||       |
    |   |    |   |___ |   |___ |       |       |   _   |       |       ||   |___  |     |
    |___|    |_______||_______||______|        |__| |__|       |______| |_______|  |___|  "
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build().mount("/", routes![index, invite_project_manager,list_developers, respond_to_invite, create_snack, list_snacks, update_snack, delete_snack, register, login]).register("/", catchers![catchers::unauthorized, catchers::not_found,
     catchers::internal_server_error])
}

