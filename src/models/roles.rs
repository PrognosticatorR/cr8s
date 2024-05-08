use crate::schema::roles;
use chrono::NaiveDateTime;
use diesel::{prelude::*, query_builder::AsChangeset};
use serde::Deserialize;

#[derive(Debug, Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = roles)]
pub struct Role {
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub id: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = roles)]
pub struct FormUser {
    pub code: Option<String>,
    pub name: Option<String>,
}
