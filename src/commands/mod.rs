use crate::auth;
use crate::schema::user_roles;
use crate::{
    models::users::NewUser,
    repositories::{role_repository::RoleRepository, user_repository::UserRepository},
};

use diesel::prelude::*;
use diesel::{Connection, ExpressionMethods, PgConnection};
fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    PgConnection::establish(&database_url).expect("unbale to connect to database")
}
pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut conn = load_db_connection();
    let hashed_password = auth::create_password_hash(password).unwrap();
    let new_user = NewUser {
        username,
        password: hashed_password,
    };
    let user = UserRepository::create(&mut conn, new_user, role_codes).unwrap();
    println!("User created: {:?}", user);
    let roles = RoleRepository::find_role_by_user(&mut conn, &user).unwrap();
    println!("Roles assigned: {:?}", roles);
}

pub fn list_users() {
    let mut conn = load_db_connection();
    let users = UserRepository::find_with_roles(&mut conn).unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub fn delete_user(id: i32) {
    let mut conn = load_db_connection();
    let _ = diesel::delete(user_roles::table.filter(user_roles::user_id.eq(id))).execute(&mut conn);
    UserRepository::delete(&mut conn, id).unwrap();
}
