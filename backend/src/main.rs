#[macro_use] extern crate rocket;
use rocket_db_pools::{diesel, Database};
mod schema;
mod database_structs;

#[derive(Database)]
#[database("postgres")]
struct Db(diesel::PgPool);

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::init()).mount("/", routes![hello])
}
