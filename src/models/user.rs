use diesel::{RunQueryDsl, BoolExpressionMethods, ExpressionMethods, PgConnection, QueryResult, QueryDsl};
use crate::schema::users;

#[derive(Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: Vec<u8>,
    pub username: String,
    pub password: Vec<u8>,
    pub email: String
}

impl User {
    pub fn clone_id(&self) -> Vec<u8> {
        self.id.clone()
    }

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

    pub fn find_email(email: String, password: Vec<u8>, connection: &PgConnection) -> Vec<User> {
        users::table
            .filter(users::dsl::email.eq(email).and(users::dsl::password.eq(password)))
            .select((users::dsl::id, users::dsl::username, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
            .expect("Error while executing query!")
    }

    pub fn find_username(username: String, password: Vec<u8>, connection: &PgConnection) -> Vec<User> {
        users::table
            .filter(users::dsl::username.eq(username).and(users::dsl::password.eq(password)))
            .select((users::dsl::id, users::dsl::username, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
            .expect("Error while executing query!")
    }

    pub fn exists_username(username: String, connection: &PgConnection) -> bool {
        users::table
            .filter(users::dsl::username.eq(username))
            .count()
            .get_result::<i64>(connection)
            .expect("Error while executing query!") >= 1
    }

    pub fn exists_username_or_email(username: String, email: String, connection: &PgConnection) -> bool {
        users::table
            .filter(users::dsl::username.eq(username).or(users::dsl::email.eq(email)))
            .count()
            .get_result::<i64>(connection)
            .expect("Error while executing query!") >= 1
    }
}