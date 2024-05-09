pub struct RoleRepository;
use crate::{
    models::roles::{NewRole, Role},
    schema::*,
};
use diesel::prelude::*;
impl RoleRepository {
    pub fn create(c: &mut PgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
    }

    pub fn find_one(conn: &mut PgConnection, role_code: &str) -> QueryResult<Role> {
        roles::table
            .filter(roles::code.eq(role_code))
            .get_result::<Role>(conn)
    }
}
