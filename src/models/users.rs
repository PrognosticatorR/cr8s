use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{prelude::*, query_builder::AsChangeset};
use serde::Deserialize;

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct FormUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub id: i32,
}
