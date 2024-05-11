use crate::{
    models::users::NewUser,
    repositories::{role_repository::RoleRepository, user_repository::UserRepository},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use diesel::{Connection, PgConnection};
fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    PgConnection::establish(&database_url).expect("unbale to connect to database")
}
pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut conn = load_db_connection();
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let hashed_password = argon.hash_password(password.as_bytes(), &salt).unwrap();
    let new_user = NewUser {
        username,
        password: hashed_password.to_string(),
    };
    let user = UserRepository::create(&mut conn, new_user, role_codes).unwrap();
    println!("User created: {:?}", user);
    let roles = RoleRepository::find_role_by_user(&mut conn, &user).unwrap();
    println!("Roles assigned: {:?}", roles);
}

pub fn list_users() {
    let mut conn = load_db_connection();
    let users = UserRepository::find_with_roles(&mut conn).unwrap();
    println!("{:?}", users)
}

pub fn delete_user(id: i32) {
    let mut conn = load_db_connection();
    UserRepository::delete(&mut conn, id).unwrap();
}
