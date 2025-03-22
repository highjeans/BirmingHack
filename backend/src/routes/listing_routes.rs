use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::{diesel::prelude::*, Connection};
use shared::{CreateListingRequest, GetListingResponse};

use crate::Db;

#[post("/", format = "json", data = "<details>")]
pub async fn create_listing(details: Json<CreateListingRequest>, mut db: Connection<Db>) -> Status {
    use crate::schema::books::dsl::*;

    let book = (
        isbn.eq(details.isbn.clone()),
        title.eq(""),
        author.eq(""),
        embeddings.eq(""),
    );

    let result = diesel::insert_into(books)
        .values(book)
        .execute(&mut db)
        .await;

    if let Err(e) = result {
        eprintln!("Error while inserting into books: {:?}", e);
        return Status::InternalServerError;
    }

    use crate::schema::booklistings::dsl::*;
    let listing = (
        user_id.eq(uuid::uuid!("550e8400-e29b-41d4-a716-446655440000")),
        book_id.eq(details.isbn.clone()),
    );

    let result = diesel::insert_into(booklistings)
        .values(listing)
        .execute(&mut db)
        .await;

    if let Err(e) = result {
        eprintln!("Error while inserting into booklistings: {:?}", e);
        return Status::InternalServerError;
    }

    Status::NoContent
}
