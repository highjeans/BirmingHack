use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupData {
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateListingRequest {
    pub isbn: String,
    pub blurb: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetListingResponse {
    pub isbn: String,
    pub blurb: String,
    pub author_id: String,
    pub author_fullname: String,
}
