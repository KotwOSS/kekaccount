use actix_web::{post, web, Responder, Result};
use serde::Deserialize;

use crate::api::http::State;
use crate::errors::actix::JsonErrorType;
use crate::models::user;
use crate::util::checker::{self, map_qres};

#[derive(Deserialize)]
pub struct DeleteData {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

#[post("/api/user/delete")]
pub async fn delete(
    delete_data: web::Json<DeleteData>,
    state: web::Data<State>,
) -> Result<impl Responder> {
    if delete_data.username.is_none() && delete_data.email.is_none() {
        return Err(JsonErrorType::MISSING_FIELD
            .new_error("Missing username or email!".to_owned())
            .into());
    }

    let password = checker::hex("password", delete_data.password.as_str(), 128)?;

    let db_connection = &checker::get_con(&state.pool)?;

    let users = match delete_data.username.clone() {
        Some(username) => map_qres(
            user::User::find_name_password(username, password, db_connection),
            "Error while selecting users",
        ),
        None => map_qres(
            user::User::find_email(delete_data.email.clone().unwrap(), password, db_connection),
            "Error while selecting users",
        ),
    }?;

    match users.into_iter().next() {
        Some(user) => {
            map_qres(
                user::User::delete(user.id, db_connection),
                "Error while deleting user",
            )?;
            Ok(web::Json(json!({
                "success": true
            })))
        }
        None => Err(JsonErrorType::BAD_CREDENTIALS
            .new_error(format!(
                "Invalid {} and or password!",
                if delete_data.username.is_some() {
                    "username"
                } else {
                    "email"
                }
            ))
            .into()),
    }
}
