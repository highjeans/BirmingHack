#[macro_use]
extern crate rocket;
use rocket_db_pools::{Database, diesel};
mod database_structs;
mod embeddings;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct Db(diesel::PgPool);

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![hello])
}
