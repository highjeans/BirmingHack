use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignupData {
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub message: String,
}
