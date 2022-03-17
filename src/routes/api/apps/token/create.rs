use hex::ToHex;
use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{app, app_token};
use crate::api::http::State;
use crate::util::checker::hex_header;
use crate::util::{random, checker::{self, map_qres}};

#[derive(Deserialize)]
pub struct CreateData {
    name: String,
    permissions: i16
}

#[post("/api/apps/{id}/token/create")]
pub async fn create(path: web::Path<(String,)>, create_data: web::Json<CreateData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let id_hex = path.into_inner().0;
    let id = checker::hex("id", id_hex.as_str(), 40)?;
    
    let token = hex_header("Authorization", 256, request.headers())?;

    let name = create_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 3, 32)?;

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b10000000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to create app tokens. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        let apps = map_qres(app::App::find_owner(id, user.id, db_connection), "Error while selecting apps")?;

        return match apps.into_iter().next() {
            Some(app)=>{
                let token = random::random_byte_array(128);
                let token_hex = token.encode_hex::<String>();

                let id = random::random_byte_array(8);
                let id_hex = id.encode_hex::<String>();


                let new_token = app_token::AppToken { id, token, name, app_id: app.id, permissions: create_data.permissions };

                map_qres(new_token.create(db_connection), "Error while inserting app token")?;

                Ok(web::Json(json!({
                    "success": true,
                    "id": id_hex,
                    "token": token_hex
                })))
            },
            None=>Err(JsonErrorType::NOT_FOUND.new_error(format!(
                "App with id '{}' not found!",
                id_hex
            )).into())
        };
    }
}
