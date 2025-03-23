use crate::database_structs::Users::{BookListings, Books, Users};
use crate::Db;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::{diesel::prelude::*, Connection};
use shared::{CreateListingRequest, CreateListingResponse, ExtractResponse, GetListingResponse};
use uuid::Uuid;

#[post("/", format = "json", data = "<details>")]
pub async fn create_listing(
    details: Json<CreateListingRequest>,
    mut db: Connection<Db>,
    user: Users,
) -> Result<Json<CreateListingResponse>, Status> {
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
        return Err(Status::InternalServerError);
    }

    let listing_id = Uuid::new_v4();
    let listing = (
        id.eq(listing_id),
        user_id.eq(user.id),
        book_id.eq(details.isbn.clone()),
    );
    let result = diesel::insert_into(booklistings)
        .values(listing)
        .execute(&mut db)
        .await;

    if let Err(e) = result {
        eprintln!("Error while inserting into booklistings: {:?}", e);
        return Err(Status::InternalServerError);
    }

    Ok(Json(CreateListingResponse {
        listing_id: listing_id.to_string(),
    }))
}

#[get("/<listing_id>")]
pub async fn get_listing(
    listing_id: String,
    user: Users,
    mut db: Connection<Db>,
) -> Result<Json<GetListingResponse>, Status> {
    println!("getting listing");
    use crate::schema::booklistings::dsl::*;
    let listing = booklistings
        .filter(id.eq(Uuid::parse_str(&listing_id).unwrap()))
        .select(BookListings::as_select())
        .load(&mut db)
        .await
        .unwrap()
        .into_iter()
        .nth(0);

    match listing {
        Some(listing) => {
            println!("listing found");

            use crate::schema::books::dsl::*;
            let book_details = books
                .filter(isbn.eq(listing.book_id))
                .select(Books::as_select())
                .load(&mut db)
                .await
                .unwrap()
                .into_iter()
                .nth(0)
                .unwrap();

            use crate::schema::users::dsl::*;
            let user_details = users
                .filter(id.eq(listing.user_id))
                .select(Users::as_select())
                .load(&mut db)
                .await
                .unwrap()
                .into_iter()
                .nth(0)
                .unwrap();

            Ok(Json(GetListingResponse {
                isbn: book_details.isbn,
                title: book_details.title,
                author: book_details.author,
                user_id: user_details.id.to_string(),
                user_fullname: user_details.fullname,
                similar_listings: vec![],
            }))
        }
        None => {
            eprintln!("listing not found");
            Err(Status::NotFound)
        }
    }
}

#[delete("/<id>")]
pub async fn delete_listing(id: String, mut db: Connection<Db>) -> Status {
    use crate::schema::booklistings::dsl::*;
    let result = diesel::delete(booklistings.filter(id.eq(id)))
        .execute(&mut db)
        .await;

    match result {
        Ok(0) => Status::NotFound,
        Ok(_) => Status::NoContent,
        Err(_) => Status::InternalServerError,
    }
}

#[post("/extract")]
pub async fn extract() -> Result<Json<ExtractResponse>, Status> {
    Ok(Json(ExtractResponse {
        blurb: "This is a blurb".to_string(),
    }))
}
