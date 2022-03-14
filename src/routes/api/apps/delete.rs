use actix_web::{post, web, Result, HttpRequest, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::app;
use crate::api::http::State;
use crate::util::checker::{self, map_qres, hex_header};

#[derive(Deserialize)]
pub struct DeleteData {
    id: String
}

#[post("/api/apps/delete")]
pub async fn delete(delete_data: web::Json<DeleteData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let id = checker::hex("id", delete_data.id.as_str(), 40)?;

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b100 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to delete apps. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        return match map_qres(app::App::delete_owner(id, user.id, db_connection), "Error while deleting app")? {
            0 => Err(JsonErrorType::NOT_FOUND.new_error(format!(
                "App with id '{}' not found!",
                delete_data.id
            )).into()),
            _ => Ok(web::Json(json!({
                "success": true
            })))
        };
    }
}
