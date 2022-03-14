use hex::ToHex;
use actix_web::{post, web, Result, HttpRequest, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::app;
use crate::api::http::State;
use crate::util::{random, checker::{self, map_qres, hex_header}};

#[derive(Deserialize)]
pub struct CreateData {
    name: String,
    description: String,
    redirect_uri: String,
    homepage: String
}

#[post("/api/apps/create")]
pub async fn create(create_data: web::Json<CreateData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let name = create_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 3, 32)?;

    let description = create_data.description.clone();
    checker::min_max_size("Length of description", description.len(), 0, 255)?;

    let redirect_uri = create_data.redirect_uri.clone();
    checker::min_max_size("Length of redirect_uri", redirect_uri.len(), 0, 255)?;
    // TODO: check validity using URL_REGEX

    let homepage = create_data.homepage.clone();
    checker::min_max_size("Length of homepage", homepage.len(), 0, 255)?;
    // TODO: check validity using URL_REGEX

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b10 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to create apps. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        let id = random::random_byte_array(20);
        let id_hex = id.encode_hex::<String>();
    
        let new_app = app::App {
            id,
            name,
            description,
            redirect_uri,
            homepage,
            owner: user.id,
        };

        map_qres(new_app.create(db_connection), "Error while inserting App")?;

        return Ok(web::Json(json!({
            "success": true,
            "id": id_hex
        })));
    }
}
