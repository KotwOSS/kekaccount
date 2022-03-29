use diesel::{RunQueryDsl, BoolExpressionMethods, ExpressionMethods, PgConnection, QueryResult, QueryDsl, PgTextExpressionMethods};
use crate::schema::users;

#[derive(Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: Vec<u8>,
    pub name: String,
    pub avatar: String,
    pub password: Vec<u8>,
    pub email: String
}

#[derive(AsChangeset)]
#[table_name="users"]
pub struct UserChangeSet {
    pub id: Option<Vec<u8>>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub password: Option<Vec<u8>>,
    pub email: Option<String>
}


impl User {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<User>> {
        users::table
            .filter(users::dsl::id.eq(id))
            .select((users::dsl::id, users::dsl::name, users::dsl::avatar, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
    }

    pub fn delete(id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(users::table)
            .filter(users::dsl::id.eq(id))
            .execute(connection)
    }

    pub fn find_email(email: String, password: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<User>> {
        users::table
            .filter(users::dsl::email.eq(email).and(users::dsl::password.eq(password)))
            .select((users::dsl::id, users::dsl::name, users::dsl::avatar, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
    }

    pub fn find_name(name: String, connection: &PgConnection) -> QueryResult<Vec<User>>{
        users::table
            .filter(users::dsl::name.eq(name))
            .select((users::dsl::id, users::dsl::name, users::dsl::avatar, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
    }

    pub fn find_name_password(name: String, password: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<User>>{
        users::table
            .filter(users::dsl::name.eq(name).and(users::dsl::password.eq(password)))
            .select((users::dsl::id, users::dsl::name, users::dsl::avatar, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
    }

    pub fn count_name(name: String, connection: &PgConnection) -> QueryResult<i64> {
        users::table
            .filter(users::dsl::name.eq(name))
            .count()
            .get_result::<i64>(connection)
    }

    pub fn count_name_or_email(name: String, email: String, connection: &PgConnection) -> QueryResult<i64> {
        users::table
            .filter(users::dsl::name.eq(name).or(users::dsl::email.eq(email)))
            .count()
            .get_result::<i64>(connection)
    }

    pub fn update(id: Vec<u8>, changes: &UserChangeSet, connection: &PgConnection) -> QueryResult<usize> {
        diesel::update(users::table)
            .filter(users::dsl::id.eq(id))
            .set(changes)
            .execute(connection)
    }

    pub fn ilike_name_ol(name: String, offset: i64, limit: i64, connection: &PgConnection) -> QueryResult<Vec<User>> {
        users::table
            .filter(users::dsl::name.ilike(name))
            .limit(limit)
            .offset(offset)
            .select((users::dsl::id, users::dsl::name, users::dsl::avatar, users::dsl::password, users::dsl::email))
            .load::<User>(connection)
    }
}