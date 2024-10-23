extern crate dotenv;
#[macro_use]
extern crate rocket;
mod routes;
mod models;
mod auth;
mod catchers;
mod db;
mod schema;

use dotenv::dotenv;
use rocket::*;
use routes::snack::create_snack;

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

    rocket::build().mount("/", routes![index, create_snack]).register("/", catchers![catchers::unauthorized])
}

