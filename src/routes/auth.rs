use crate::auth;
use crate::{
    repositories::user_repository::UserRepository,
    routes::{CacheConn, DbConn},
};
use auth::Credentials;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::{
    post,
    serde::json::{json, Json, Value},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    db: DbConn,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let username = credentials.username.clone();
    let user = db
        .run(move |conn| {
            UserRepository::find_by_username(conn, &username)
                .map_err(|err| super::serve_error(&err.into()))
        })
        .await?;
    let session_id = auth::authorize_user(&user, &credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("wrong credentials")))?;

    cache
        .set_ex::<_, _, ()>(format!("session/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({"token":session_id}))
        .map_err(|err| super::serve_error(&err.into()))
}

pub fn logout() {
    unimplemented!()
}
