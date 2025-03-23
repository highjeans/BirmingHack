#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use rocket_db_pools::{diesel, Database};
mod database_structs;
mod embeddings;
mod routes;
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
    dotenv().ok();
    std::env::var("JWT_KEY").unwrap();
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![hello])
        .mount("/users", routes![user_routes::login, user_routes::signup])
        .mount("/listings", routes![listing_routes::create_listing])
}
