use rocket_db_pools::diesel::prelude::*;
use uuid::Uuid;
use crate::schema::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[derive(Queryable, Selectable, Insertable, Associations)]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(table_name = books)]
pub struct Books {
    pub isbn: i64,
    pub title: String,
    pub author: String,
    pub embeddings: String,
    pub user_id: Uuid,
}

#[derive(Queryable, Selectable, Insertable, Associations)]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(table_name = socials)]
pub struct Socials {
    pub id: Uuid,
    pub platform: String,
    pub username: String,
    pub user_id: Uuid,
}
