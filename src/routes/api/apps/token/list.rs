use hex::ToHex;

use actix_web::{post, web, Result, Responder, HttpRequest};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{app_token, app};
use crate::api::http::State;
use crate::util::checker::{self, map_qres, hex_header};

#[derive(Deserialize)]
pub struct ListData {
    // unused for now
}

#[post("/api/apps/{id}/token/list")]
pub async fn list(path: web::Path<(String,)>, _list_data: web::Json<ListData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let id_hex = path.into_inner().0;
    let id = checker::hex("id", id_hex.as_str(), 40)?;
    
    let token = hex_header("Authorization", 256, request.headers())?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b1000000000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to list tokens. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        let apps = map_qres(app::App::find_owner(id, user.id, db_connection), "Error while selecting apps")?;

        return match apps.into_iter().next() {
            Some(app)=>{
                let tokens = map_qres(app_token::AppToken::find_app(app.id, db_connection), "Error while selecting app tokens")?;
                
                let mapped: Vec<serde_json::Value> = tokens.into_iter()
                    .map(|tk| json!({
                        "id": tk.id.encode_hex::<String>(),
                        "perms": tk.permissions,
                        "name": tk.name
                    }))
                    .collect();
        
                Ok(web::Json(mapped))
            },
            None=>Err(JsonErrorType::NOT_FOUND.new_error(format!(
                "App with id '{}' not found!",
                id_hex
            )).into())
        };
    }
}
