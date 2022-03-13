use diesel::{PgConnection, QueryResult, QueryDsl, RunQueryDsl, ExpressionMethods};

use crate::schema::apps;

#[derive(Queryable, Insertable)]
#[table_name="apps"]
pub struct App {
    pub id: Vec<u8>,
    pub owner: Vec<u8>,
    pub name: String,
    pub description: String,
    pub redirect_uri: String,
}

impl App {
    pub fn clone_id(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub fn clone_owner(&self) -> Vec<u8> {
        self.owner.clone()
    }

    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(apps::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: Vec<u8>, connection: &PgConnection) -> QueryResult<Vec<App>> {
        apps::table
            .filter(apps::dsl::id.eq(id))
            .select((apps::dsl::id, apps::dsl::owner, apps::dsl::name, apps::dsl::description, apps::dsl::redirect_uri))
            .load::<App>(connection)
    }
}