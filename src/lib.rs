#![allow(dead_code)]

extern crate diesel;
extern crate rocket;

mod auth;
pub mod commands;
mod models;
mod repositories;
pub mod routes;
mod schema;
mod tests;
mod utils;
