use super::super::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = rustaceans)]
pub struct FormRustacean {
    pub name: Option<String>,
    pub email: Option<String>,
}
