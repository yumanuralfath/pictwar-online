#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod schema;
mod utils;

use crate::routes::routes::index;
use crate::routes::users::{create_user, get_user, get_users};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_user, get_users, create_user])
        .attach(utils::db::attach_db())
}
