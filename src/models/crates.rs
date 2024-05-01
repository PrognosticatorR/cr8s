use super::super::schema::*;
use super::rustaceans::Rustacean;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Rustacean))]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name=crates)]

pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name=crates)]
pub struct FormCrate {
    pub rustacean_id: Option<i32>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
}