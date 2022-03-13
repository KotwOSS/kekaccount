use hex::{ToHex};

use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::errors::actix::JsonErrorType;
use crate::models::{token};
use crate::api::http::State;
use crate::util::{self, checker::{self, map_qres, map_opt}};

#[derive(Deserialize)]
pub struct ListData {
    // unused for now
    // offset: Option<u32>,
    // amount: Option<u8>
}

#[derive(Serialize)]
pub struct ShowAbleToken {
    id: String,
    name: String,
    active: bool
}

#[post("/api/auth/token/list")]
pub async fn list(_list_data: web::Json<ListData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let headers = request.headers();

    let authorization_hex = map_opt(headers.get("Authorization"), "Missing authorization header")?.to_str().unwrap();

    let token =  util::hex::parse_to_buf(authorization_hex, 256)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing authorize header: {}",
            e
        )))?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b10 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to list tokens. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        let tokens = map_qres(token::Token::find_user(user.id, db_connection), "Error while selecting tokens")?;
        let mapped: Vec<ShowAbleToken> = tokens.iter()
            .map(|tk| ShowAbleToken {
                id: tk.id.encode_hex::<String>(),
                name: tk.name.clone(),
                active: tk.id==token.id
            })
            .collect();

        return Ok(web::Json(mapped));
    }
}
