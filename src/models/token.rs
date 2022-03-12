use diesel::{PgConnection, QueryResult, QueryDsl, BoolExpressionMethods, RunQueryDsl, ExpressionMethods};

use crate::schema::tokens;

#[derive(Queryable, Insertable)]
#[table_name="tokens"]
pub struct Token {
    pub id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub name: String,
    pub permissions: i16
}

impl Token {
    pub fn clone_id(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(tokens::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> Vec<Token> {
        tokens::table
            .filter(tokens::dsl::id.eq(id))
            .select((tokens::dsl::id, tokens::dsl::user_id, tokens::dsl::name, tokens::dsl::permissions))
            .load::<Token>(connection)
            .expect("Error while executing query!")
    }

    pub fn get_name(user_id: Vec<u8>, name: String, connection: &PgConnection) -> Vec<Token> {
        tokens::table
            .filter(tokens::dsl::name.eq(name).and(tokens::dsl::user_id.eq(user_id)))
            .select((tokens::dsl::id, tokens::dsl::user_id, tokens::dsl::name, tokens::dsl::permissions))
            .load::<Token>(connection)
            .expect("Error while executing query!")
    }
}