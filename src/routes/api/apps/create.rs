//use hex::ToHex;
use actix_web::{post, web, Result, HttpRequest, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
//use crate::models::{user, token};
use crate::api::http::State;
use crate::util::{self, checker::{self, map_opt}};

#[derive(Deserialize)]
pub struct CreateData {
    name: String,
    description: String,
    redirect_uri: String
}

#[post("/api/apps/create")]
pub async fn create(create_data: web::Json<CreateData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let headers = request.headers();

    let authorization_hex = map_opt(headers.get("Authorization"), "Missing authorization header")?.to_str().unwrap();

    let token =  util::hex::parse_to_buf(authorization_hex, 256)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing authorize header: {}",
            e
        )))?;

    let name = create_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 3, 32)?;

    let _description = create_data.description.clone();
    checker::min_max_size("Length of description", name.len(), 0, 255)?;

    let _redirect_uri = create_data.redirect_uri.clone();
    checker::min_max_size("Length of redirect_uri", name.len(), 0, 255)?;
    // TODO: check validity using URL_REGEX

    let db_connection = &checker::get_con(&state.pool)?;

    let (_user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b1 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to create apps. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        return Ok("kekw");
    }
}
