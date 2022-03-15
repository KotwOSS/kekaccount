use std::io::Error;
use std::sync::Arc;

use actix_web::{HttpServer, App, web};

use crate::database::PgPool;
use crate::colors;
use crate::routes;

pub struct State {
    pub pool: Arc<PgPool>
}

pub async fn main(pool: Arc<PgPool>, address: String, port: u16) -> Result<(), Error> {
    println!("{}START{} http on {}{}:{}", colors::LIGHT_BLUE, colors::RESET, colors::ORANGE,  address, port);
    match HttpServer::new(move || {
        App::new()
            .service(routes::api::index)
            // TOKEN
            .service(routes::api::auth::token::create::create)
            .service(routes::api::auth::token::list::list)
            .service(routes::api::auth::token::delete::delete)
            .service(routes::api::auth::token::info::info)
            // REGISTER
            .service(routes::api::auth::register::register)
            // APPS
            .service(routes::api::apps::create::create)
            .service(routes::api::apps::delete::delete)
            .service(routes::api::apps::list::list)
            .service(routes::api::apps::update::update)
            .service(routes::api::apps::search::search)
            // USERS
            .service(routes::api::users::retrieve::retrieve)
            .service(routes::api::users::search::search)
            .app_data(web::Data::new(State { pool: pool.clone() }))
    })
    .bind((address, port))?
    .run()
    .await {
        Ok(v) => {
            println!("{}STOP{} http", colors::ORANGE, colors::RESET);
            Ok(v)
        },
        Err(error) => {
            println!("{}ERROR{} while binding http: {}", colors::RED, colors::RESET, error);
            return Err(error);
        }
    }
}