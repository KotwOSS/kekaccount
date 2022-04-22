#![feature(once_cell)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

extern crate dotenv;

use dotenv::dotenv;
use std::{env, sync::Arc};

pub mod api;
pub mod colors;
pub mod database;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;

use database::PgPool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("{}START{} kekaccount", colors::BLUE, colors::RESET);

    let pool: PgPool =
        database::establish_connection(env::var("DATABASE_URL").expect("Database url not set!"));

    let http_address = env::var("http_address").unwrap_or("0.0.0.0".to_owned());

    let http_port = env::var("http_port")
        .unwrap_or("5070".to_owned())
        .parse()
        .unwrap_or(5070);

    let tcp_address = env::var("tcp_address").unwrap_or("0.0.0.0".to_owned());

    let tcp_port = env::var("tcp_port")
        .unwrap_or("5071".to_owned())
        .parse()
        .unwrap_or(5071);

    let pool_arc = Arc::new(pool.clone());

    let smtp_host = env::var("smtp_host").unwrap_or("youremail.com".to_owned());

    let smtp_port = env::var("smtp_port")
        .unwrap_or("25".to_owned())
        .parse()
        .unwrap_or(25);

    let smtp_user = env::var("smtp_user").unwrap_or("account".to_owned());

    let smtp_password = env::var("smtp_password").unwrap_or("1234".to_owned());

    let smtp_from =
        env::var("smtp_from").unwrap_or("KekAccount <account@youremail.com>".to_owned());

    let verification_base =
        env::var("verification_base").unwrap_or("http://localhost:3000/verify?id=".to_owned());

    // INITIALIZATION
    util::checker::init();
    api::smtp::init(
        smtp_host,
        smtp_port,
        smtp_user,
        smtp_password,
        smtp_from,
        verification_base,
    );

    let (_tcp, _http) = tokio::join!(
        api::tcp::main(&pool, tcp_address, tcp_port),
        api::http::main(pool_arc, http_address, http_port)
    );
}
