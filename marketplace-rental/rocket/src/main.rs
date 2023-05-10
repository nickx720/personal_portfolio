#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::http::Status;
use rocket_contrib::databases::database;
use rocket_contrib::json::JsonValue;

mod models;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
