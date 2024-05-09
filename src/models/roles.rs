use crate::schema::roles;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;


#[derive(Queryable, Debug)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = roles)]
pub struct FormUser {
    pub code: Option<String>,
    pub name: Option<String>,
}
