use rocket_db_pools::diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub fullname: String,
}
