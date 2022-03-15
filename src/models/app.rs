use diesel::{PgConnection, QueryResult, QueryDsl, RunQueryDsl, ExpressionMethods, BoolExpressionMethods};

use crate::schema::apps;

#[derive(Queryable, Insertable)]
#[table_name="apps"]
pub struct App {
    pub id: Vec<u8>,
    pub owner: Vec<u8>,
    pub name: String,
    pub description: String,
    pub redirect_uri: String,
    pub homepage: String,
}

#[derive(AsChangeset)]
#[table_name="apps"]
pub struct AppChangeSet {
    pub id: Option<Vec<u8>>,
    pub owner: Option<Vec<u8>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub redirect_uri: Option<String>,
    pub homepage: Option<String>,
}

impl App {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(apps::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<App>> {
        apps::table
            .filter(apps::dsl::id.eq(id))
            .select((apps::dsl::id, apps::dsl::owner, apps::dsl::name, apps::dsl::description, apps::dsl::redirect_uri, apps::dsl::homepage))
            .load::<App>(connection)
    }

    pub fn find_owner(owner: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<App>> {
        apps::table
            .filter(apps::dsl::owner.eq(owner))
            .select((apps::dsl::id, apps::dsl::owner, apps::dsl::name, apps::dsl::description, apps::dsl::redirect_uri, apps::dsl::homepage))
            .load::<App>(connection)
    }

    pub fn delete(id: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(apps::table)
            .filter(apps::dsl::id.eq(id))
            .execute(connection)
    }

    pub fn delete_owner(id: Vec<u8>, owner: Vec<u8>, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(apps::table)
            .filter(apps::dsl::id.eq(id).and(apps::dsl::owner.eq(owner)))
            .execute(connection)
    }

    pub fn update_owner(id: Vec<u8>, owner: Vec<u8>, changes: &AppChangeSet, connection: &PgConnection) -> QueryResult<usize> {
        diesel::update(apps::table)
            .filter(apps::dsl::id.eq(id).and(apps::dsl::owner.eq(owner)))
            .set(changes)
            .execute(connection)
    }
}