use hex::ToHex;

use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::api::http::State;
use crate::util::{self, checker::{self, map_opt}};

#[derive(Deserialize)]
pub struct InfoData {
    // unused for now
}

#[post("/api/auth/token/info")]
pub async fn info(_info_data: web::Json<InfoData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let headers = request.headers();

    let authorization_hex = map_opt(headers.get("Authorization"), "Missing authorization header")?.to_str().unwrap();

    let token =  util::hex::parse_to_buf(authorization_hex, 256)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing authorize header: {}",
            e
        )))?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let (user, token) = checker::authorize(token, db_connection)?;

    Ok(web::Json(json!({
        "user": {
            "name": user.username,
            "email": user.email,
            "id": user.id.encode_hex::<String>(),
        },
        "token": {
            "id": token.id.encode_hex::<String>(),
            "permissions": token.permissions,
            "name": token.name,
        },
        "success": true
    })))
}
