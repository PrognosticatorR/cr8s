use diesel::{
    query_dsl::methods::LimitDsl, ExpressionMethods, PgConnection, QueryDsl, QueryResult,
    RunQueryDsl,
};

use crate::{models::rustaceans::*, schema::*};
pub struct RustacenRepository;

impl RustacenRepository {
    pub fn find(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        QueryDsl::order(
            LimitDsl::limit(rustaceans::table, limit),
            rustaceans::id.desc(),
        )
        .load::<Rustacean>(conn)
    }
    pub fn find_one(conn: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(conn)
    }
    pub fn create(conn: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        let _ = diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(conn);
        let last_id = Self::last_inserted_id(conn)?;
        Self::find_one(conn, last_id)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(conn)
    }
    pub fn update(
        conn: &mut PgConnection,
        update_data: FormRustacean,
        id: i32,
    ) -> QueryResult<Rustacean> {
        let _ = diesel::update(rustaceans::table.find(id))
            .set(update_data)
            .execute(conn);
        Self::find_one(conn, id)
    }

    pub fn last_inserted_id(conn: &mut PgConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(conn)
    }
}
