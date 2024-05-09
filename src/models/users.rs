use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize)]
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
