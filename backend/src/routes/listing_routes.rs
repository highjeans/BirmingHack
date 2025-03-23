use crate::Db;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::{diesel::prelude::*, Connection};
use shared::{CreateListingRequest, GetListingResponse};
use uuid::Uuid;

#[post("/", format = "json", data = "<details>")]
pub async fn create_listing(details: Json<CreateListingRequest>, mut db: Connection<Db>) -> Status {
    use crate::schema::booklistings::dsl::*;
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

    let listing = (
        user_id.eq(Uuid::parse_str("e8580a1a-862c-488d-9ba4-f67f79233b20").unwrap()),
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

#[get("/<id>")]
pub async fn get_listing(
    id: String,
    mut db: Connection<Db>,
) -> Result<Json<GetListingResponse>, Status> {
    use crate::schema::booklistings::dsl::*;
    use crate::schema::books::dsl::*;
    use crate::schema::users::dsl::*;

    let listing = booklistings
        .filter(book_id.eq(id.clone()))
        .first(&mut db)
        .await;

    match listing {
        Ok(listing) => {
            // Fetch the book details based on the book_id
            let book_details = books.filter(isbn.eq(id)).first::<Books>(&mut db).await;

            match book_details {
                Ok(book) => {
                    // You may also want to fetch the user details, but it's not needed in the current example
                    Ok(Json(GetListingResponse {
                        isbn: book.isbn,
                        title: book.title,
                        author: book.author,
                        user_id: listing.user_id,
                        user_fullname: listing.user_fullname,
                    }))
                }
                Err(_) => Err(Status::NotFound),
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[delete("/<id>")]
pub async fn delete_listing(id: String, mut db: Connection<Db>) -> Status {
    use crate::schema::booklistings::dsl::*;

    let result = diesel::delete(booklistings.filter(book_id.eq(id)))
        .execute(&mut db)
        .await;

    match result {
        Ok(0) => Status::NotFound,
        Ok(_) => Status::NoContent,
        Err(_) => Status::InternalServerError,
    }
}
