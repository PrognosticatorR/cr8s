pub struct UserRepository;
use crate::{
    models::{
        roles::NewRole,
        user_roles::NewUserRole,
        users::{NewUser, User},
    },
    schema::*,
};
use diesel::prelude::*;

use super::role_repository::RoleRepository;
impl UserRepository {
    pub fn create(
        c: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;
        for code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_one(c, &code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: code.to_owned(),
                        name: code,
                    };
                    let role = RoleRepository::create(c, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };
            let _ = diesel::insert_into(user_roles::table)
                .values(new_user_role)
                .execute(c);
        }
        Ok(user)
    }
    pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users::table.limit(10).load(conn)
    }
}
