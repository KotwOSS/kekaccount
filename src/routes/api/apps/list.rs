use hex::ToHex;

use actix_web::{post, web, Result, HttpRequest, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::app;
use crate::api::http::State;
use crate::util::checker::{self, map_qres, hex_header};

#[derive(Deserialize)]
pub struct ListData {
    // unused for now
}

#[post("/api/apps/list")]
pub async fn list(_list_data: web::Json<ListData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b10000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to list apps. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        let apps = map_qres(app::App::find_owner(user.id, db_connection), "Error while selecting apps")?;

        let mapped: Vec<serde_json::Value> = apps.into_iter()
            .map(|app| json!({
                "id": app.id.encode_hex::<String>(),
                "name": app.name,
                "description": app.description,
                "redirect_uri": app.redirect_uri,
                "homepage": app.homepage
            }))
            .collect();

        return Ok(web::Json(mapped));
    }
}
