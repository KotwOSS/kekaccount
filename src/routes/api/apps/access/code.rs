use hex::ToHex;

use actix_web::{post, web, HttpRequest, Responder, Result};
use serde::Deserialize;

use crate::api::http::State;
use crate::errors::actix::JsonErrorType;
use crate::models::{access_code, token};
use crate::util::checker::{self, hex_header, map_qres};

#[derive(Deserialize)]
pub struct CodeData {
    code: String,
}

#[post("/api/apps/access/code")]
pub async fn code(
    code_data: web::Json<CodeData>,
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;

    let (app, _token) = checker::authorize_app(token, db_connection)?;

    let code_id = checker::hex("Access code", code_data.code.as_str(), 64)?;

    let codes = map_qres(
        access_code::AccessCode::find_app(code_id, app.id, db_connection),
        "Error while selecting access codes",
    )?;

    match codes.into_iter().next() {
        Some(code) => {
            map_qres(
                access_code::AccessCode::delete(code.id, db_connection),
                "Error while deleting access code",
            )?;

            let access_tokens = map_qres(
                token::Token::find(code.token_id, db_connection),
                "Error while selecting tokens",
            )?;

            match access_tokens.into_iter().next() {
                Some(access_token) => Ok(web::Json(json!({
                    "success": true,
                    "id": access_token.id.encode_hex::<String>(),
                    "perms": access_token.permissions,
                    "token": access_token.token.encode_hex::<String>(),
                    "name": access_token.name
                }))),
                None => Err(JsonErrorType::NOT_FOUND
                    .new_error(format!(
                        "Access code with id '{}' not found!",
                        code_data.code
                    ))
                    .into()),
            }
        }
        None => Err(JsonErrorType::NOT_FOUND
            .new_error(format!(
                "Access code with id '{}' not found!",
                code_data.code
            ))
            .into()),
    }
}
