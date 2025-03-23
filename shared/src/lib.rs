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

#[derive(Serialize, Deserialize)]
pub struct CreateListingRequest {
    pub isbn: String,
    pub blurb: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetListingResponse {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub user_id: String,
    pub user_fullname: String,
}
