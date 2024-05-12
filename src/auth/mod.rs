use rand::{distributions::Alphanumeric, thread_rng, Rng};

use argon2::{
    password_hash::{self, rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use serde::Deserialize;

use crate::models::users::User;

#[derive(Deserialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(
    user: &User,
    credentials: &Credentials,
) -> Result<String, password_hash::Error> {
    let db_password_hash = PasswordHash::new(&user.password).unwrap();
    let argon = Argon2::default();
    argon.verify_password(credentials.password.as_bytes(), &db_password_hash)?;
    let session_token = thread_rng()
        .sample_iter(Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    Ok(session_token)
}

pub fn create_password_hash(password: String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let hashed_password = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(hashed_password.to_string())
}
