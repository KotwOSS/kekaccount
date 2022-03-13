use actix_web::{Responder, get};

pub mod auth;
pub mod apps;

#[get("/api")]
pub async fn index() -> impl Responder {
    "Hello world!"
}