use actix_web::{Responder, get};

pub mod auth;
pub mod apps;
pub mod users;
pub mod user;

#[get("/api")]
pub async fn index() -> impl Responder {
    "Hello world!"
}