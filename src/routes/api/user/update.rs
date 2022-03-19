use hex::ToHex;
use actix_web::{post, web, Result, HttpRequest, Responder};
use serde::Deserialize;

use crate::api::smtp;
use crate::errors::actix::JsonErrorType;
use crate::models::{user, verification};
use crate::api::http::State;
use crate::util::checker::{self, map_qres, hex_header};
use crate::util::random;

#[derive(Deserialize)]
pub struct UpdateData {
    name: Option<String>,
    email: Option<String>
}

#[post("/api/user/update")]
pub async fn update(update_data: web::Json<UpdateData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let mut update_count = 0;


    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;


    let mut name: Option<String> = update_data.name.clone();
    if let Some(ref rname) = name { 
        checker::min_max_size("Length of name", rname.len(), 3, 32)?;
        if user.name.eq(rname) { 
            name = None;
        } else { update_count+=1;  }
    }

    let mut email: Option<String> = update_data.email.clone();
    if let Some(ref remail) = email { 
        checker::min_max_size("Length of email", remail.len(), 3, 32)?;
        checker::email("Email", remail.as_str())?;
        if user.email.eq(remail) { 
            email = None;
        } else { update_count+=1;  }
    }

    if update_count==0 {
        return Err(JsonErrorType::BAD_REQUEST.new_error(format!(
            "You have to add atleast one field which you want to update"
        )).into());
    }

    if token.permissions & 0b1000000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to update yourself. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        // Resend verification email on email change
        if let Some(ref email) = email {
            map_qres(verification::Verification::delete_owner_all(user.id.clone(), db_connection), "Error while deleting verifications")?;

            let verification_id = random::random_byte_array(128);
            let verification_id_hex = verification_id.encode_hex::<String>();

            let new_verification = verification::Verification { id: verification_id, owner: user.id.clone() };
            map_qres(new_verification.create(db_connection), "Error while creating verification entry")?;

            smtp::send_verification(user.name.as_str(), email.as_str(), verification_id_hex)
                .await.map_err(|e| JsonErrorType::INTERNAL_SERVER_ERROR.new_error(format!(
                    "Sending verification email failed: {}",
                    e
                )))?;
        }


        map_qres(user::User::update(user.id, &user::UserChangeSet {
            id: None,
            password: None,
            email,
            name
        }, db_connection), "Error while updating user")?;

        return Ok(web::Json(json!({
            "success": true,
            "updated_fields": update_count
        })));
    }
}
