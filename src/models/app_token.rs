use diesel::{PgConnection, QueryResult, QueryDsl, RunQueryDsl, ExpressionMethods};

use crate::schema::app_tokens;

#[derive(Queryable, Insertable)]
#[table_name="app_tokens"]
pub struct AppToken {
    pub id: Vec<u8>,
    pub token: Vec<u8>,
    pub app_id: Vec<u8>,
    pub name: String,
    pub permissions: i16
}

impl AppToken {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(app_tokens::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<AppToken>> {
        app_tokens::table
            .filter(app_tokens::dsl::id.eq(id))
            .select((app_tokens::dsl::id, app_tokens::dsl::token, app_tokens::dsl::app_id, app_tokens::dsl::name, app_tokens::dsl::permissions))
            .load::<AppToken>(connection)
    }

    pub fn delete(id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(app_tokens::table)
            .filter(app_tokens::dsl::id.eq(id))
            .execute(connection)
    }

    pub fn find_app(app_id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<AppToken>> {
        app_tokens::table
            .filter(app_tokens::dsl::app_id.eq(app_id))
            .select((app_tokens::dsl::id, app_tokens::dsl::token, app_tokens::dsl::app_id, app_tokens::dsl::name, app_tokens::dsl::permissions))
            .load::<AppToken>(connection)
    }
}