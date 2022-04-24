use hex::ToHex;

use actix_web::{post, web, HttpRequest, Responder, Result};
use serde::Deserialize;

use crate::api::http::State;
use crate::util::checker::{self, hex_header};

#[derive(Deserialize)]
pub struct InfoData {
    // unused for now
}

#[post("/api/apps/token/info")]
pub async fn info(
    _info_data: web::Json<InfoData>,
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;

    let (app, token) = checker::authorize_app(token, db_connection)?;

    Ok(web::Json(json!({
        "app_id": app.id.encode_hex::<String>(),
        "token": {
            "id": token.id.encode_hex::<String>(),
            "permissions": token.permissions,
            "name": token.name,
        },
        "success": true
    })))
}
