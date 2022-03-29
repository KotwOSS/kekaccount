use actix_web::{post, web, Result, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user, verification};
use crate::api::http::State;
use crate::util::checker::{self, map_qres};

#[derive(Deserialize)]
pub struct VerifyData {
    username: Option<String>,
    email: Option<String>,
    password: String,
    id: String
}

#[post("/api/auth/verify")]
pub async fn verify(verify_data: web::Json<VerifyData>, state: web::Data<State>) -> Result<impl Responder> {
    if verify_data.username.is_none() && verify_data.email.is_none() {
        return Err(JsonErrorType::MISSING_FIELD.new_error("Missing username or email!".to_owned()).into());
    }

    let password = checker::hex("password", verify_data.password.as_str(), 128)?;
    let id = checker::hex("id", verify_data.id.as_str(), 256)?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let users = match verify_data.username.clone() {
        Some(username) => map_qres(user::User::find_name_password(username, password, db_connection), "Error while selecting users"),
        None => map_qres(user::User::find_email(verify_data.email.clone().unwrap(), password, db_connection), "Error while selecting users")
    }?;

    match users.into_iter().next() {
        Some(user) => {
            let verifications = map_qres(verification::Verification::find_owner(id, user.id, db_connection), "Error while selecting verifications")?;

            match verifications.into_iter().next() {
                Some(verification) => {
                    map_qres(verification::Verification::delete(verification.id, db_connection), "Error while deleting verification")?;

                    Ok(web::Json(json!({
                        "success": true
                    })))
                },
                None => Err(JsonErrorType::NOT_FOUND.new_error(format!(
                    "Verification with id '{}' not found!",
                    verify_data.id
                )).into())
            }
        },
        None => Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
                "Invalid {} and or password!", 
                if verify_data.username.is_some() {"username"} else {"email"}
            )).into())
    }
}
