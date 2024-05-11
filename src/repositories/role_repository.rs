pub struct RoleRepository;
use crate::{
    models::{
        roles::{NewRole, Role},
        user_roles::UserRole,
        users::User,
    },
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

    pub fn find_role_by_user(conn: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(user).get_results(conn)?;

        Self::find_role_by_ids(
            conn,
            user_roles
                .iter()
                .map(|user_role: &UserRole| user_role.role_id)
                .collect(),
        )
    }
    pub fn find_role_by_ids(conn: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results::<Role>(conn)
    }
}
