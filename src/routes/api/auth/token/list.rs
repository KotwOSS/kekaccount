use hex::ToHex;

use actix_web::{post, web, HttpRequest, Responder, Result};
use serde::Deserialize;

use crate::api::http::State;
use crate::errors::actix::JsonErrorType;
use crate::models::token;
use crate::util::checker::{self, hex_header, map_qres};

#[derive(Deserialize)]
pub struct ListData {
    // unused for now
// offset: Option<u32>,
// amount: Option<u8>
}

#[post("/api/auth/token/list")]
pub async fn list(
    _list_data: web::Json<ListData>,
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b1 == 0 {
        return Err(JsonErrorType::FORBIDDEN
            .new_error(format!(
                "You don't have the permissions to list tokens. (Your permission level: {})",
                token.permissions
            ))
            .into());
    } else {
        let tokens = map_qres(
            token::Token::find_user(user.id, db_connection),
            "Error while selecting tokens",
        )?;

        let mapped: Vec<serde_json::Value> = tokens
            .into_iter()
            .map(|tk| {
                json!({
                    "id": tk.id.encode_hex::<String>(),
                    "name": tk.name,
                    "perms": tk.permissions,
                    "active": tk.id==token.id
                })
            })
            .collect();

        Ok(web::Json(mapped))
    }
}
