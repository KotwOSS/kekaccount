use diesel::{PgConnection, QueryResult, QueryDsl, BoolExpressionMethods, RunQueryDsl, ExpressionMethods};

use crate::schema::tokens;

#[derive(Queryable, Insertable)]
#[table_name="tokens"]
pub struct Token {
    pub id: Vec<u8>,
    pub token: Vec<u8>,
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

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<Token>> {
        tokens::table
            .filter(tokens::dsl::id.eq(id))
            .select((tokens::dsl::id, tokens::dsl::token, tokens::dsl::user_id, tokens::dsl::name, tokens::dsl::permissions))
            .load::<Token>(connection)
    }

    pub fn delete(id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(tokens::table)
            .filter(tokens::dsl::id.eq(id))
            .execute(connection)       
    }

    pub fn delete_user(user_id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(tokens::table)
            .filter(tokens::dsl::user_id.eq(user_id))
            .execute(connection)       
    }

    pub fn find_token(token: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<Token>> {
        tokens::table
            .filter(tokens::dsl::token.eq(token))
            .select((tokens::dsl::id, tokens::dsl::token, tokens::dsl::user_id, tokens::dsl::name, tokens::dsl::permissions))
            .load::<Token>(connection)
    }

    pub fn find_user(user_id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<Token>> {
        tokens::table
            .filter(tokens::dsl::user_id.eq(user_id))
            .select((tokens::dsl::id, tokens::dsl::token, tokens::dsl::user_id, tokens::dsl::name, tokens::dsl::permissions))
            .load::<Token>(connection)
    }

    pub fn get_name(user_id: Vec<u8>, name: String, connection: &PgConnection) -> QueryResult<Vec<Token>> {
        tokens::table
            .filter(tokens::dsl::name.eq(name).and(tokens::dsl::user_id.eq(user_id)))
            .select((tokens::dsl::id, tokens::dsl::token, tokens::dsl::user_id, tokens::dsl::name, tokens::dsl::permissions))
            .load::<Token>(connection)
    }
}