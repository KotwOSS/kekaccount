use hex::ToHex;

use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::Deserialize;

use crate::api::http::State;
use crate::errors::actix::JsonErrorType;
use crate::models::verification;
use crate::util::checker::{self, hex_header, map_qres};

#[derive(Deserialize)]
pub struct InfoData {
    // unused for now
}

#[post("/api/user/info")]
pub async fn info(_info_data: web::Json<InfoData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b100000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to list private user data. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        let verifications = map_qres(verification::Verification::count_owner(user.id.clone(), db_connection), "Error while counting verifications")?;

        return Ok(web::Json(json!({
            "name": user.name,
            "email": user.email,
            "id": user.id.encode_hex::<String>(),
            "verified": verifications==0,
            "success": true
        })));
    }
}
