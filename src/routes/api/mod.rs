use actix_web::{Responder, get};

pub mod auth;

#[get("/api")]
pub async fn index() -> impl Responder {
    "Hello world!"
}