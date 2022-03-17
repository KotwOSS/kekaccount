use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{app, app_token};
use crate::api::http::State;
use crate::util::checker::hex_header;
use crate::util::checker::{self, map_qres};

#[derive(Deserialize)]
pub struct DeleteData {
    id: String
}

#[post("/api/apps/{id}/token/delete")]
pub async fn delete(path: web::Path<(String,)>, delete_data: web::Json<DeleteData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let id_hex = path.into_inner().0;
    let id = checker::hex("id", id_hex.as_str(), 40)?;
    
    let token = hex_header("Authorization", 256, request.headers())?;

    let token_id = checker::hex("token_id", delete_data.id.as_str(), 16)?;

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b100000000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to delete app tokens. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        let apps = map_qres(app::App::find_owner(id, user.id, db_connection), "Error while selecting apps")?;

        return match apps.into_iter().next() {
            Some(app)=>{
                match map_qres(app_token::AppToken::delete_app(token_id, app.id, db_connection), "Error while deleting app token")? {
                    0 => Err(JsonErrorType::NOT_FOUND.new_error(format!(
                        "App token with id '{}' not found!",
                        delete_data.id
                    )).into()),
                    _ => Ok(web::Json(json!({
                        "success": true
                    })))
                }
            },
            None=>Err(JsonErrorType::NOT_FOUND.new_error(format!(
                "App with id '{}' not found!",
                id_hex
            )).into())
        };
    }
}
