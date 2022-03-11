use diesel::{PgConnection, QueryResult, QueryDsl};
use crate::diesel::RunQueryDsl;

use crate::schema::users;

use crate::diesel::ExpressionMethods;

#[derive(Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: Vec<u8>,
    pub username: String,
    pub password: Vec<u8>,
    pub email: String
}

impl User {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> Vec<User> {
        users::table
            .filter(users::dsl::id.eq(id))
            .select((users::dsl::id, users::dsl::username, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
            .expect("Error while executing query!")
    }
}