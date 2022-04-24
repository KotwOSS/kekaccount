use actix_web::{get, Responder};

pub mod apps;
pub mod auth;
pub mod user;
pub mod users;

#[get("/api")]
pub async fn index() -> impl Responder {
    "Hello world!"
}
