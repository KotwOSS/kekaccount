use std::io::Error;

use actix_web::{HttpServer, App, web};

use crate::database::PgPool;
use crate::colors;
use crate::routes;

pub struct State {
    pub pool: PgPool
}

pub async fn main(pool: PgPool, address: String, port: u16) -> Result<(), Error> {
    println!("{}START{} http on {}{}:{}", colors::LIGHT_BLUE, colors::RESET, colors::ORANGE,  address, port);

    match HttpServer::new(move || {
        App::new()
            .service(routes::api::index)
            .service(routes::api::auth::token::create::create)
            .app_data(web::Data::new(State { pool: pool.clone() }))
    })
    .bind((address, port))?
    .run()
    .await {
        Ok(v) => Ok(v),
        Err(error) => {
            println!("{}ERROR{} while binding http: {}", colors::RED, colors::RESET, error);
            return Err(error);
        }
    }
}