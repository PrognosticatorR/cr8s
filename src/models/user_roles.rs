use diesel::associations::Associations;
use diesel::deserialize::Queryable;
use diesel::prelude::Insertable;

use super::roles::Role;
use super::users::User;
use crate::schema::user_roles;

#[derive(Associations, Queryable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=user_roles)]
pub struct UserRole {
    pub id: i32,
    pub role_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=user_roles)]
pub struct NewUserRole {
    pub role_id: i32,
    pub user_id: i32,
}
