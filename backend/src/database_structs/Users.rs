use crate::{
    routes::user_routes::Claims,
    schema::{self, users::dsl as user_table},
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};
use rocket_db_pools::diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::users)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Users {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(jwt) = req.headers().get("Authorization").next() {
            let mut validation = Validation::new(Algorithm::HS256);
            validation.set_required_spec_claims(&["expires_at", "id"]);
            let data = match decode::<Claims>(
                jwt,
                &DecodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_bytes()),
                &validation,
            ) {
                Ok(c) => c,
                Err(_) => {
                    return request::Outcome::Error((
                        Status::Unauthorized,
                        "Invalid JWT".to_string(),
                    ))
                }
            };
            let user_id_res = Uuid::parse_str(data.claims.id.as_str());

            if let Err(_) = user_id_res {
                return request::Outcome::Error((Status::Unauthorized, "Invalid JWT".to_string()));
            }

            let db_guard = req.guard::<rocket_db_pools::Connection<crate::Db>>().await;

            if let request::Outcome::Error(err) = db_guard {
                return request::Outcome::Error((
                    err.0,
                    "Unable to connect to database".to_string(),
                ));
            }

            let mut db = db_guard.unwrap();

            use user_table::*;

            if let Ok(user) = users.find(user_id_res.unwrap()).first(&mut db).await {
                request::Outcome::Success(user)
            } else {
                request::Outcome::Error((Status::Unauthorized, "No user found".to_string()))
            }
        } else {
            request::Outcome::Error((Status::Unauthorized, "No JWT provided".to_string()))
        }
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::books)]
pub struct Books {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub embeddings: String,
}

#[derive(Queryable, Selectable, Insertable, Associations)]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(table_name = schema::socials)]
pub struct Socials {
    pub id: Uuid,
    pub platform: String,
    pub username: String,
    pub user_id: Uuid,
}

#[derive(Queryable, Selectable, Insertable, Associations)]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(belongs_to(Books, foreign_key = book_id))]
#[diesel(table_name = schema::booklistings)]
pub struct BookListings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: String,
}
