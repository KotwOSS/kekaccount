use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::token;
use crate::api::http::State;
use crate::util::checker::{self, map_qres, hex_header};

#[derive(Deserialize)]
pub struct TerminateData {
    // unused for now
}

#[post("/api/auth/token/terminate")]
pub async fn terminate(_terminate_data: web::Json<TerminateData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let token = checker::authorize_token(token, db_connection)?;

    if token.permissions & 0b10000000000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to terminate yourself. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        map_qres(token::Token::delete(token.id, db_connection), "Error while deleting token")?;

        Ok(web::Json(json!({
            "success": true
        })))
    }
}
