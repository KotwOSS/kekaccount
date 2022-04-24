use hex::ToHex;

use actix_web::{post, web, Responder, Result};
use serde::Deserialize;

use crate::api::http::State;
use crate::errors::actix::JsonErrorType;
use crate::models::user;
use crate::util::checker::{self, map_qres};

#[derive(Deserialize)]
pub struct InfoData {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

#[post("/api/auth/info")]
pub async fn info(
    info_data: web::Json<InfoData>,
    state: web::Data<State>,
) -> Result<impl Responder> {
    if info_data.username.is_none() && info_data.email.is_none() {
        return Err(JsonErrorType::MISSING_FIELD
            .new_error("Missing username or email!".to_owned())
            .into());
    }

    let password = checker::hex("password", info_data.password.as_str(), 128)?;

    let db_connection = &checker::get_con(&state.pool)?;

    let users = match info_data.username.clone() {
        Some(username) => map_qres(
            user::User::find_name_password(username, password, db_connection),
            "Error while selecting users",
        ),
        None => map_qres(
            user::User::find_email(info_data.email.clone().unwrap(), password, db_connection),
            "Error while selecting users",
        ),
    }?;

    match users.into_iter().next() {
        Some(user) => {
            return Ok(web::Json(json!({
                "success": true,
                "id": user.id.encode_hex::<String>(),
                "name": user.name,
                "email": user.email
            })))
        }
        None => Err(JsonErrorType::BAD_CREDENTIALS
            .new_error(format!(
                "Invalid {} and or password!",
                if info_data.username.is_some() {
                    "username"
                } else {
                    "email"
                }
            ))
            .into()),
    }
}
