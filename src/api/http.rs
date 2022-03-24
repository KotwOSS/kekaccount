use std::io::Error;
use std::sync::Arc;

use actix_cors::Cors;
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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);


        App::new()
            // CORS
            .wrap(cors)

            // INDEX
            .service(routes::api::index)
            // TOKEN
            .service(routes::api::auth::token::create::create)
            .service(routes::api::auth::token::list::list)
            .service(routes::api::auth::token::delete::delete)
            .service(routes::api::auth::token::info::info)
            .service(routes::api::auth::token::terminate::terminate)
            // ACCOUNT
            .service(routes::api::auth::register::register)
            .service(routes::api::auth::verify::verify)
            // APPS
            .service(routes::api::apps::create::create)
            .service(routes::api::apps::delete::delete)
            .service(routes::api::apps::list::list)
            .service(routes::api::apps::update::update)
            .service(routes::api::apps::search::search)
            .service(routes::api::apps::retrieve::retrieve)
            // APP TOKENS
            .service(routes::api::apps::token::create::create)
            .service(routes::api::apps::token::delete::delete)
            .service(routes::api::apps::token::list::list)
            .service(routes::api::apps::token::info::info)
            // USERS
            .service(routes::api::users::retrieve::retrieve)
            .service(routes::api::users::search::search)
            // USER
            .service(routes::api::user::info::info)
            .service(routes::api::user::update::update)
            .service(routes::api::user::delete::delete)
            // ACCESS CODES
            .service(routes::api::apps::access::generate::generate)
            .service(routes::api::apps::access::code::code)
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