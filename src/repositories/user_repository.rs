pub struct UserRepository;
use crate::{
    models::{
        roles::{NewRole, Role},
        user_roles::{NewUserRole, UserRole},
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

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(conn)?;
        diesel::delete(user_roles::table.filter(user_roles::user_id.eq(id))).execute(conn)
    }

    pub fn find_with_roles(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load(conn)?;
        let result = user_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(conn)?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }
}
