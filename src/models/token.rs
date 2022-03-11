use diesel::{PgConnection, QueryResult, QueryDsl};
use crate::diesel::RunQueryDsl;

use crate::schema::tokens;

use crate::diesel::ExpressionMethods;

#[derive(Queryable, Insertable)]
#[table_name="tokens"]
pub struct Token {
    pub id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub name: String,
    pub permissions: i16
}

impl Token {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(tokens::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> Vec<Token> {
        tokens::table
            .filter(tokens::dsl::id.eq(id))
            .select((tokens::dsl::id, tokens::dsl::username, tokens::dsl::password))
            .load::<Token>(connection)
            .expect("Error while executing query!")
    }
}