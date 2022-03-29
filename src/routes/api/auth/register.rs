use hex::{ToHex};
use actix_web::{post, web, Result, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user, verification};
use crate::api::{smtp, http::State};
use crate::util::{self, random, checker::{self, map_qres}};

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    email: String,
    avatar: String,
    password: String
}

#[post("/api/auth/register")]
pub async fn register(register_data: web::Json<RegisterData>, state: web::Data<State>) -> Result<impl Responder> {

    let password = util::hex::parse_to_buf(register_data.password.as_str(), 128)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing field password! ({})",
            e
        )))?;

    let username = register_data.username.clone();
    checker::min_max_size("The length of username", username.len(), 3, 32)?;

    let email = register_data.email.clone();
    checker::min_max_size("Length of email", email.len(), 3, 32)?;
    checker::email("The field email", email.as_str())?;

    let avatar = register_data.avatar.clone();
    checker::min_max_size("The length of avatar", avatar.len(), 0, 255)?;

    let db_connection = &checker::get_con(&state.pool)?;

    match map_qres(user::User::count_name_or_email(username.clone(), email.clone(), db_connection), "Error while selecting users")? {
        0 => {
            let id = random::random_byte_array(16);
            let id_hex = id.encode_hex::<String>();


            let verification_id = random::random_byte_array(128);
            let verification_id_hex = verification_id.encode_hex::<String>();

            let new_verification = verification::Verification { id: verification_id, owner: id.clone() };
            map_qres(new_verification.create(db_connection), "Error while creating verification entry")?;

            smtp::send_verification(username.as_str(), email.as_str(), verification_id_hex)
                .await.map_err(|e| JsonErrorType::INTERNAL_SERVER_ERROR.new_error(format!(
                    "Sending verification email failed: {}",
                    e
                )))?;


            let new_user = user::User { name: username, email, avatar, password, id };
            map_qres(new_user.create(db_connection), "Error while inserting user")?;


            Ok(web::Json(json!({
                "success": true,
                "id": id_hex,
                "verification_email": "sent"
            })))
        },
        _ => {
            Err(JsonErrorType::EXISTS.new_error(format!(
                "A user with that name or email already exists!"
            )).into())
        }
    }
}
