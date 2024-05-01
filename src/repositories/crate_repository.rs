use diesel::{
    query_dsl::methods::LimitDsl, ExpressionMethods, PgConnection, QueryDsl, QueryResult,
    RunQueryDsl,
};

use crate::{models::crates::*, schema::*};
pub struct CrateRepository;

impl CrateRepository {
    pub fn create(conn: &mut PgConnection, records: NewCrate) -> QueryResult<Crate> {
        let _ = diesel::insert_into(crates::table)
            .values(records)
            .execute(conn);
        let last_insterted = Self::last_inserted_id(conn)?;
        Self::find_one(conn, last_insterted)
    }
    pub fn update(conn: &mut PgConnection, values: FormCrate, id: i32) -> QueryResult<Crate> {
        let _ = diesel::update(crates::table.find(id))
            .set(values)
            .execute(conn);
        Self::find_one(conn, id)
    }
    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn)
    }
    pub fn find(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        QueryDsl::order(LimitDsl::limit(crates::table, limit), crates::id.desc())
            .load::<Crate>(conn)
    }
    pub fn find_one(conn: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(conn)
    }
    fn last_inserted_id(conn: &mut PgConnection) -> QueryResult<i32> {
        crates::table
            .select(crates::id)
            .order(crates::id.desc())
            .first(conn)
    }
}
