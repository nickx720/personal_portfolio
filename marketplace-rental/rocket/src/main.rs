use models::{create, list, run_migrations};
use rocket::fairing::AdHoc;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

mod models;
mod schema;

#[allow(dead_code)]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[database("diesel_db")]
pub struct Db(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_migrations))
        .mount("/", routes![create, list])
}
