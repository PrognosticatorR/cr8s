use diesel::{Connection, PgConnection};

use crate::{models::users::NewUser, repositories::user_repository::UserRepository};
fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    PgConnection::establish(&database_url).expect("unbale to connect to database")
}
pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let new_user = NewUser { username, password };
    let mut conn = load_db_connection();
    let user = UserRepository::create(&mut conn, new_user, role_codes);
    println!("User created: {:?}", user);
}

pub fn list_users() {}

pub fn delete_user(_id: i32) {}
