#[macro_use]
extern crate rocket;

mod routes;
use routes::home::index;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
