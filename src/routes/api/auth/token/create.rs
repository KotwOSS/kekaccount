use hex::{ToHex};
use actix_web::{post, web, Result};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user, token};
use crate::api::http::State;
use crate::util::{self, random, checker};

#[derive(Deserialize)]
pub struct CreateData {
    username: Option<String>,
    email: Option<String>,
    password: String,
    name: String,
    permissions: i16
}

#[post("/api/auth/token/create")]
pub async fn create(create_data: web::Json<CreateData>, state: web::Data<State>) -> Result<String> {
    if create_data.username.is_none() && create_data.email.is_none() {
        return Err(JsonErrorType::MISSING_FIELD.new_error("Missing username or email!".to_owned()).into());
    }

    let password = util::hex::parse_to_buf(create_data.password.clone(), 128)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing field password! ({})",
            e
        )))?;

    let name = create_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 3, 32)?;

    let db_connection = &state.pool.get()
        .expect("Error while connecting to database!");
    
    let users = match create_data.username.clone() {
        Some(username) => user::User::find_username(username, password, db_connection),
        None => user::User::find_email(create_data.email.clone().unwrap(), password, db_connection)
    };


    if users.len() == 1 {
        let user_id = users[0].clone_id();

        let tokens = token::Token::get_name(user_id.clone(), name.clone(), db_connection);

        if tokens.len() == 0 {
            let id = random::random_byte_array(128);
            let id_hex = id.encode_hex::<String>();

            let new_token = token::Token {
                id,
                name,
                user_id,
                permissions: create_data.permissions
            };

            new_token.create(db_connection).map_err(|e| JsonErrorType::INTERNAL_SERVER_ERROR.new_error(format!(
                "Error while trying to insert token. Please report this to an administrator! ({})",
                e
            )))?;

            return Ok(json!({
                "success": true,
                "created": true,
                "token": id_hex
            }).to_string());
        } else {
            return Ok(json!({
                "success": true,
                "created": false,
                "token": tokens[0].clone_id().encode_hex::<String>()
            }).to_string());
        }
    } else {
        return Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
            "Invalid {} and or password!", 
            if create_data.username.is_some() {"username"} else {"email"}
        )).into());
    }
}
