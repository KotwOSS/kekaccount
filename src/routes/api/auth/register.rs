use hex::{ToHex};
use actix_web::{post, web, Result};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user};
use crate::api::http::State;
use crate::util::{self, random, checker};

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    email: String,
    password: String
}

#[post("/api/auth/register")]
pub async fn register(register_data: web::Json<RegisterData>, state: web::Data<State>) -> Result<String> {
    let password = util::hex::parse_to_buf(register_data.password.clone(), 128)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing field password! ({})",
            e
        )))?;

    let username = register_data.username.clone();
    checker::min_max_size("The length of username", username.len(), 3, 32)?;

    let email = register_data.email.clone();
    checker::min_max_size("Length of email", email.len(), 3, 32)?;
    checker::email("The field email", email.as_str())?;

    let db_connection = &state.pool.get()
        .expect("Error while connecting to database!");

    if user::User::exists_username_or_email(username.clone(), email.clone(), db_connection) {
        return Err(JsonErrorType::EXISTS.new_error(format!(
            "A user with that name or email already exists!"
        )).into());
    } else {
        let id = random::random_byte_array(16);
        let id_hex = id.encode_hex::<String>();

        let new_user = user::User {
            username,
            email,
            password,
            id
        };

        new_user.create(db_connection).map_err(|e| JsonErrorType::INTERNAL_SERVER_ERROR.new_error(format!(
            "Error while trying to insert user. Please report this to an administrator! ({})",
            e
        )))?;

        return Ok(json!({
            "success": true,
            "user_id": id_hex
        }).to_string());
    }
}