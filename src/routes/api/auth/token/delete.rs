use actix_web::{post, web, Result, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user, token};
use crate::api::http::State;
use crate::util::{self, checker::{self, map_qres}};

#[derive(Deserialize)]
pub struct DeleteData {
    username: Option<String>,
    email: Option<String>,
    password: String,
    id: String
}

#[post("/api/auth/token/delete")]
pub async fn delete(delete_data: web::Json<DeleteData>, state: web::Data<State>) -> Result<impl Responder> {
    if delete_data.username.is_none() && delete_data.email.is_none() {
        return Err(JsonErrorType::MISSING_FIELD.new_error("Missing username or email!".to_owned()).into());
    }

    let password = util::hex::parse_to_buf(delete_data.password.as_str(), 128)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing field password! ({})",
            e
        )))?;

    let id = util::hex::parse_to_buf(delete_data.id.as_str(), 16)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing field password! ({})",
            e
        )))?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let users = match delete_data.username.clone() {
        Some(username) => map_qres(user::User::find_name(username, password, db_connection), "Error while selecting users"),
        None => map_qres(user::User::find_email(delete_data.email.clone().unwrap(), password, db_connection), "Error while selecting users")
    }?;

    match users.into_iter().next() {
        Some(_user) => {
            match  map_qres(token::Token::delete(id, db_connection), "Error while deleting token")? {
                0 => Err(JsonErrorType::NOT_FOUND.new_error(format!(
                    "Token with id '{}' not found!",
                    delete_data.id
                )).into()),
                _ => Ok(web::Json(json!({
                    "success": true
                })))
            }
        },
        None => Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
                "Invalid {} and or password!", 
                if delete_data.username.is_some() {"username"} else {"email"}
            )).into())
    }
}
