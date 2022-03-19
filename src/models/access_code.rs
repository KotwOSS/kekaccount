use diesel::{PgConnection, QueryResult, QueryDsl, RunQueryDsl, ExpressionMethods, BoolExpressionMethods};

use crate::schema::access_codes;

#[derive(Queryable, Insertable)]
#[table_name="access_codes"]
pub struct AccessCode {
    pub id: Vec<u8>,
    pub app_id: Vec<u8>,
    pub token_id: Vec<u8>
}

impl AccessCode {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(access_codes::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<AccessCode>> {
        access_codes::table
            .filter(access_codes::dsl::id.eq(id))
            .select((access_codes::dsl::id, access_codes::dsl::app_id, access_codes::dsl::token_id))
            .load::<AccessCode>(connection)
    }

    pub fn delete(id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(access_codes::table)
            .filter(access_codes::dsl::id.eq(id))
            .execute(connection)
    }

    pub fn delete_app(id: Vec<u8>, app_id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(access_codes::table)
            .filter(access_codes::dsl::id.eq(id).and(access_codes::dsl::app_id.eq(app_id)))
            .execute(connection)
    }

    pub fn delete_app_all(app_id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(access_codes::table)
            .filter(access_codes::dsl::app_id.eq(app_id))
            .execute(connection)
    }

    pub fn find_app(id: Vec<u8>, app_id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<AccessCode>> {
        access_codes::table
            .filter(access_codes::dsl::id.eq(id).and(access_codes::dsl::app_id.eq(app_id)))
            .select((access_codes::dsl::id, access_codes::dsl::app_id, access_codes::dsl::token_id))
            .load::<AccessCode>(connection)
    }

    pub fn find_token(token_id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<AccessCode>> {
        access_codes::table
            .filter(access_codes::dsl::token_id.eq(token_id))
            .select((access_codes::dsl::id, access_codes::dsl::app_id, access_codes::dsl::token_id))
            .load::<AccessCode>(connection)
    }
}