#[macro_use]
extern crate rocket;
use rocket_db_pools::{Database, diesel};
mod database_structs;
mod routes;
mod embeddings;
mod schema;

use routes::*;

#[derive(Database)]
#[database("postgres")]
pub struct Db(diesel::PgPool);

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    std::env::var("JWT_KEY").unwrap();
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![hello])
        .mount("/users", routes![user_routes::login, user_routes::signup])
}
