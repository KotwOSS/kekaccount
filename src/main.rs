#[macro_use] 
extern crate diesel;

#[macro_use] 
extern crate serde_json;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub mod api;
pub mod models;
pub mod schema;
pub mod database;
pub mod colors;
pub mod routes;
pub mod random;
pub mod errors;

use database::PgPool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!(
        "{}START{} kekaccount", 
        colors::BLUE, 
        colors::RESET
    );

    let pool: PgPool = database::establish_connection(env::var("DATABASE_URL").expect("Database url not set!"));


    let http_address = env::var("http_address")
        .unwrap_or("0.0.0.0".to_owned());

    let http_port = env::var("http_port")
        .unwrap_or("5070".to_owned())
        .parse()
        .unwrap_or(5070);

    let tcp_address = env::var("tcp_address")
        .unwrap_or("0.0.0.0".to_owned());

    let tcp_port = env::var("tcp_port")
        .unwrap_or("5071".to_owned())
        .parse()
        .unwrap_or(5071);

    let cloned_pool = pool.clone();


    let (_tcp, _http) = tokio::join!(
        api::tcp::main(&pool, tcp_address, tcp_port),
        async move {
            api::http::main(cloned_pool, http_address, http_port).await
        }
    );
}
