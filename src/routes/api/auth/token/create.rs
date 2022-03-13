use hex::{ToHex};
use actix_web::{post, web, Result, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user, token};
use crate::api::http::State;
use crate::util::{self, random, checker::{self, map_qres}};

#[derive(Deserialize)]
pub struct CreateData {
    username: Option<String>,
    email: Option<String>,
    password: String,
    name: String,
    permissions: i16
}

#[post("/api/auth/token/create")]
pub async fn create(create_data: web::Json<CreateData>, state: web::Data<State>) -> Result<impl Responder> {
    if create_data.username.is_none() && create_data.email.is_none() {
        return Err(JsonErrorType::MISSING_FIELD.new_error("Missing username or email!".to_owned()).into());
    }

    let password = util::hex::parse_to_buf(create_data.password.as_str(), 128)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing field password! ({})",
            e
        )))?;

    let name = create_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 3, 32)?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let users = match create_data.username.clone() {
        Some(username) => map_qres(user::User::find_username(username, password, db_connection), "Error while selecting users"),
        None => map_qres(user::User::find_email(create_data.email.clone().unwrap(), password, db_connection), "Error while selecting users")
    }?;

    match users.first() {
        Some(user) => {
            let user_id = user.clone_id();

            let token = random::random_byte_array(128);
            let token_hex = token.encode_hex::<String>();

            let id = random::random_byte_array(8);
            let id_hex = id.encode_hex::<String>();


            let new_token = token::Token { id, token, name, user_id, permissions: create_data.permissions };

            map_qres(new_token.create(db_connection), "Error while inserting token")?;

            Ok(web::Json(json!({
                "success": true,
                "token": token_hex,
                "id": id_hex
            })))
        },
        None => Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
                "Invalid {} and or password!", 
                if create_data.username.is_some() {"username"} else {"email"}
            )).into())
    }
}
