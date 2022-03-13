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
            .service(routes::api::auth::token::create::create)
            .service(routes::api::auth::token::list::list)
            .service(routes::api::auth::token::delete::delete)
            .service(routes::api::auth::register::register)
            .service(routes::api::apps::create::create)
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