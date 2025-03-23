use crate::database_structs::Users::Users;
use crate::Db;
use bcrypt::BcryptError;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::CookieJar;
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::{diesel::prelude::*, Connection};
use shared::{LoginData, LoginResponse, SignupData};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    expires_at: u128,
    id: String,
}

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
        fullname.eq(details.fullname.clone()),
    );
    match diesel::insert_into(users)
        .values(user_details)
        .execute(&mut db)
        .await
    {
        Ok(_) => Status::NoContent,
        Err(_) => Status::InternalServerError,
    }
}

#[post("/login", format = "json", data = "<details>")]
pub async fn login(
    details: Json<LoginData>,
    mut db: Connection<Db>,
    cookies: &CookieJar<'_>,
) -> (Status, Json<LoginResponse>) {
    use crate::schema::users::dsl::*;
    match users
        .filter(username.eq(details.username.clone()))
        .select(Users::as_select())
        .load(&mut db)
        .await
    {
        Ok(user_list) => {
            if user_list.is_empty() {
                return (
                    Status::Forbidden,
                    Json(LoginResponse {
                        message: "Invalid username".to_string(),
                    }),
                );
            }
            let user = user_list.get(0).unwrap();
            let claims = Claims {
                expires_at: (SystemTime::now() + Duration::new(60 * 60 * 24 * 7, 0))
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                id: user.id.to_string(),
            };
            match bcrypt::verify(details.password.clone(), user.password.as_str()) {
                Ok(matched) => {
                    if !matched {
                        return (
                            Status::Forbidden,
                            Json(LoginResponse {
                                message: "Invalid Password".to_string(),
                            }),
                        );
                    }
                    match encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_bytes()),
                    ) {
                        Ok(jwt) => {
                            cookies.add_private(("jwt", jwt.clone()));
                            (
                                Status::Ok,
                                Json(LoginResponse {
                                    message: "".to_string(),
                                }),
                            )
                        }
                        Err(_) => (
                            Status::InternalServerError,
                            Json(LoginResponse {
                                message: "Could not create jwt.".to_string(),
                            }),
                        ),
                    }
                }
                Err(err) => (
                    Status::InternalServerError,
                    Json(LoginResponse {
                        message: err.to_string(),
                    }),
                ),
            }
        }
        Err(err) => (
            Status::InternalServerError,
            Json(LoginResponse {
                message: err.to_string(),
            }),
        ),
    }
}
