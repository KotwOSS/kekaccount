
use hex;
use actix_web::{post, web, Result};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user};
use crate::api::http::State;
//use crate::random;

#[derive(Deserialize)]
pub struct CreateData {
    username: String,
    password: String,
    name: String,
    permissions: i16
}

#[post("/api/auth/token/create")]
pub async fn create(create_data: web::Json<CreateData>, state: web::Data<State>) -> Result<&'static str> {
    println!("KEKW: {}", create_data.password.clone());

    let password_decoded = hex::decode(create_data.password.clone())
        .map_err(|e| 
            JsonErrorType::BAD_REQUEST.new_error(format!("Password must be in hex format! ({})", e))
        )?;

    let db_connection = &state.pool.get()
        .expect("Error while connecting to database!");
    
    let users = user::User::find(password_decoded, db_connection);


    // let id = random::random_byte_array(128);
    // let id_hex = id.encode_hex::<String>();

    println!(
        "TOKEN_CREATE: 
            found: {} users

            username: {},
            password: {},
            name: {},
            permissions: {}
        ", 
        users.len(),
        create_data.username, 
        create_data.password,
        create_data.name,
        create_data.permissions
    );
    
    Ok("kekw")
}
