use rocket::{serde::json::Json, http::Status};
use rocket_db_pools::{Connection, diesel::prelude::*};
use shared::{LoginData, SignupData};
use crate::database_structs::Users::Users;

use crate::Db;

#[post("/signup", format = "json", data = "<details>")]
pub async fn signup(details: Json<SignupData>, mut db: Connection<Db>) -> Status {
    use crate::schema::users::dsl::*;
    let password_hash_res = bcrypt::hash(details.password.clone(), 10);
    if let Err(_) = password_hash_res {
        return Status::InternalServerError;
    }
    let password_hash = password_hash_res.unwrap();
    let user_details = (
        id.eq(uuid::Uuid::new_v4()),
        username.eq(details.username.clone()),
        password.eq(password_hash),
        fullname.eq(details.fullname.clone())
    );
    match diesel::insert_into(users).values(user_details).execute(&mut db).await {
        Ok(_) => Status::NoContent,
        Err(_) => Status::InternalServerError,
    }
}

#[post("/login", format = "json", data = "<details>")]
pub async fn login(details: Json<LoginData>, mut db: Connection<Db>) {
    use crate::schema::users::dsl::*;
    match users.filter(username.eq(details.username.clone())).select(Users::as_select()).load(&mut db).await {
        Ok(user) => {
            if user.is_empty() {
                // TODO: return that the user was not found
                todo!();
            }
            match bcrypt::verify(details.password.clone(), user.get(0).unwrap().password.as_str()) {
                Ok(_) => {
                    // TODO: Create secure cookie and return message
                    todo!()
                },
                Err(_) => {
                    // TODO: Return that password is invalid
                    todo!();
                },
            }
        },
        Err(err) => {
            let res = err.to_string();
            // TODO: return error message with InternalServerError
        },
    }
}
