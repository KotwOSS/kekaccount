use hex::ToHex;

use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::Deserialize;

use crate::api::http::State;
use crate::util::checker::{self, hex_header};

#[derive(Deserialize)]
pub struct InfoData {
    // unused for now
}

#[post("/api/auth/token/info")]
pub async fn info(_info_data: web::Json<InfoData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let (user, token) = checker::authorize(token, db_connection)?;

    Ok(web::Json(json!({
        "user_id": user.id.encode_hex::<String>(),
        "token": {
            "id": token.id.encode_hex::<String>(),
            "permissions": token.permissions,
            "name": token.name,
        },
        "success": true
    })))
}
