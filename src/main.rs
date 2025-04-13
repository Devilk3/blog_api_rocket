#[macro_use]
extern crate rocket;

mod db;
mod models;
mod schema;
mod routes;

use crate::db::establish_connection;
use routes::user::create_user;
use routes::post::create_post;
//use rocket::serde::json::Json;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok(); // Load .env

    rocket::build()
        .manage(establish_connection()) // DB pool
        .mount("/api", routes![create_user, create_post])
}
