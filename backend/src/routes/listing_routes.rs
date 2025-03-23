use std::collections::HashMap;

use crate::database_structs::Users::{BookListings, Books, Users};
use crate::Db;
use base64::{decode, encode};
use rocket::serde::json::serde_json;
use rocket::serde::Deserialize;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::{diesel::prelude::*, Connection};
use shared::{CreateListingRequest, CreateListingResponse, ExtractResponse, GetListingResponse};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Data {
    details: Option<BookDetails>,
}

#[derive(Deserialize, Debug)]
struct BookDetails {
    title: Option<String>,
    author: Option<Vec<String>>,
}

fn extract_title(book_details: &Data) -> String {
    println!("{:?}", book_details);
    book_details
        .details
        .as_ref()
        .unwrap()
        .title
        .as_ref()
        .unwrap_or(&"Title not available".to_string())
        .to_string()
}

fn extract_author(book_details: &Data) -> String {
    println!("{:?}", book_details);
    book_details
        .details
        .as_ref()
        .unwrap()
        .author
        .as_ref()
        .and_then(|authors| authors.get(0)) // Get the first author
        .unwrap_or(&"Author not available".to_string())
        .to_string()
}

#[post("/", format = "json", data = "<details>")]
pub async fn create_listing(
    details: Json<CreateListingRequest>,
    mut db: Connection<Db>,
    user: Users,
) -> Result<Json<CreateListingResponse>, Status> {
    use crate::embeddings::produce_embeddings;
    use crate::schema::booklistings::dsl::*;
    use crate::schema::books::dsl::*;

    let other_embeddings = produce_embeddings(&details.blurb).unwrap_or_default();
    let mut s: Vec<String> = Vec::new();
    for (i, val) in other_embeddings.iter().enumerate() {
        if i > 0 {
            s.push(String::from(" "));
        }
        s.push(format!("{}", val));
    }

    println!(
        "https://openlibrary.org/api/books?bibkeys=ISBN:{}&jscmd=details&format=json",
        details.isbn,
    );

    let response = reqwest::Client::new()
        .get(format!(
            "https://openlibrary.org/api/books?bibkeys=ISBN:{}&jscmd=details&format=json",
            details.isbn
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", response); // Print the raw response to debug

    let other_isbn: std::collections::HashMap<String, Data> =
        serde_json::from_str(&response).unwrap();

    println!("parsed: {:?}", other_isbn);

    let book_details = other_isbn.get(&format!("ISBN:{}", details.isbn)).unwrap();
    let the_title = extract_title(book_details);
    let the_author = extract_author(book_details);

    println!("title: {:?}, author: {:?}", the_title, the_author);

    let book = (
        isbn.eq(details.isbn.clone()),
        title.eq(the_title),
        author.eq(the_author),
        embeddings.eq(s.concat()),
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
        .next();

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

            let all_details: Vec<Books> = books
                .select(Books::as_select())
                .load(&mut db)
                .await
                .unwrap()
                .into_iter()
                .collect();

            let book_embedding_pairs: HashMap<Books, Vec<f32>> = all_details
                .into_iter()
                .map(|b| (b.clone(), base64_to_embedding(&b.embeddings)))
                .collect();

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

fn base64_to_embedding(s: &str) -> Vec<f32> {
    s.split(" ").filter_map(|w| w.parse::<f32>().ok()).collect()
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
