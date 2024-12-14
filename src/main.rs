#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use crate::routes::get_routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", get_routes())
        .attach(utils::db::attach_db())
}
