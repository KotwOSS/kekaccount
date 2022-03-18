use diesel::{PgConnection, QueryResult, QueryDsl, RunQueryDsl, ExpressionMethods, BoolExpressionMethods};

use crate::schema::verifications;

#[derive(Queryable, Insertable)]
#[table_name="verifications"]
pub struct Verification {
    pub id: Vec<u8>,
    pub owner: Vec<u8>,
}

impl Verification {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(verifications::table)
            .values(self)
            .execute(connection)
    }

    pub fn find_owner_all(owner: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<Verification>> {
        verifications::table
            .filter(verifications::dsl::owner.eq(owner))
            .select((verifications::dsl::id, verifications::dsl::owner))
            .load::<Verification>(connection)
    }

    pub fn find_owner(id: Vec<u8>, owner: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<Verification>> {
        verifications::table
            .filter(verifications::dsl::id.eq(id).and(verifications::dsl::owner.eq(owner)))
            .select((verifications::dsl::id, verifications::dsl::owner))
            .load::<Verification>(connection)
    }

    pub fn delete(id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(verifications::table)
            .filter(verifications::dsl::id.eq(id))
            .execute(connection)
    }

    pub fn delete_owner(id: Vec<u8>, owner: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(verifications::table)
            .filter(verifications::dsl::id.eq(id).and(verifications::dsl::owner.eq(owner)))
            .execute(connection)
    }
}