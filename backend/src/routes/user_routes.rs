use bcrypt::BcryptError;
use rocket::http::CookieJar;
use rocket::{serde::{Deserialize, Serialize, json::Json}, http::Status};
use rocket_db_pools::{Connection, diesel::prelude::*};
use shared::{LoginData, LoginResponse, SignupData, SignupResponse};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::database_structs::Users::Users;
use crate::Db;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub expires_at: u128,
    pub id: String
}

#[post("/signup", format = "json", data = "<details>")]
pub async fn signup(details: Json<SignupData>, mut db: Connection<Db>) -> (Status, Json<SignupResponse>) {
    use crate::schema::users::dsl::*;

    if let Ok(user_list) = users.filter(username.eq(details.username.clone())).select(Users::as_select()).load(&mut db).await {
        if !user_list.is_empty() {
            return (Status::ImATeapot, Json(SignupResponse {
                message: "You already have an account, please login.".to_string()
            }))
        }
    };

    let password_hash_res = bcrypt::hash(details.password.clone(), 10);
    if let Err(_) = password_hash_res {
        return (Status::InternalServerError, Json(SignupResponse {
            message: "Failed to hash the password".to_string()
        }));
    }
    let password_hash = password_hash_res.unwrap();
    let user_details = (
        id.eq(uuid::Uuid::new_v4()),
        username.eq(details.username.clone()),
        password.eq(password_hash),
        fullname.eq(details.fullname.clone())
    );
    match diesel::insert_into(users).values(user_details).execute(&mut db).await {
        Ok(_) => (Status::NoContent, Json(SignupResponse {
            message: "".to_string()
        })),
        Err(_) => (Status::InternalServerError, Json(SignupResponse {
            message: "Failed to create user for an unexpected reason".to_string()
        })),
    }
}

#[post("/login", format = "json", data = "<details>")]
pub async fn login(details: Json<LoginData>, mut db: Connection<Db>) -> (Status, Json<LoginResponse>) {
    use crate::schema::users::dsl::*;
    match users.filter(username.eq(details.username.clone())).select(Users::as_select()).load(&mut db).await {
        Ok(user_list) => {
            if user_list.is_empty() {
                return (Status::Forbidden, Json(LoginResponse {
                    message: "Invalid username".to_string(),
                }));
            }
            let user = user_list.get(0).unwrap();
            let claims = Claims {
                expires_at: (SystemTime::now() + Duration::new(60*60*24*7, 0)).duration_since(UNIX_EPOCH).unwrap().as_millis(),
                id: user.id.to_string()
            };
            match bcrypt::verify(details.password.clone(), user.password.as_str()) {
                Ok(matched) => {
                    if !matched {
                        return (Status::Forbidden, Json(LoginResponse {
                            message: "Invalid Password".to_string(),
                        }));
                    }
                    match encode(&Header::default(), &claims, &EncodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_bytes())) {
                        Ok(jwt) => {
                            (Status::Ok, Json(LoginResponse {
                                message: jwt.clone()
                            }))
                        },
                        Err(_) => {
                            (Status::InternalServerError, Json(LoginResponse {
                                message: "Could not create jwt.".to_string(),
                            }))
                        },
                    }
                },
                Err(err) => {
                    (Status::InternalServerError, Json(LoginResponse {
                        message: err.to_string(),
                    }))
                },
            }
        },
        Err(err) => {
            (Status::InternalServerError, Json(LoginResponse {
                message: err.to_string(),
            }))
        },
    }
}
