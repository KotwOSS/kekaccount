use actix_web::{post, web, Responder, Result};
use hex::ToHex;
use serde::Deserialize;

use crate::api::http::State;
use crate::models::app;
use crate::util::checker::{self, map_qres};

#[derive(Deserialize)]
pub struct SearchData {
    name: String,
    offset: Option<i64>,
    limit: Option<i64>,
}

#[post("/api/apps/search")]
pub async fn search(
    search_data: web::Json<SearchData>,
    state: web::Data<State>,
) -> Result<impl Responder> {
    let name = search_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 0, 32)?;

    let limit = search_data.limit.clone().unwrap_or(10);
    checker::min_max_size("Limit", limit.try_into().unwrap(), 0, 20)?;

    let offset = search_data.offset.clone().unwrap_or(0);

    let db_connection = &checker::get_con(&state.pool)?;

    let apps = map_qres(
        app::App::ilike_name_ol(format!("%{}%", name), offset, limit, db_connection),
        "Error while selecting apps",
    )?;

    let mapped: Vec<serde_json::Value> = apps
        .into_iter()
        .map(|app| {
            json!({
                "id": app.id.encode_hex::<String>(),
                "name": app.name,
                "description": app.description,
                "owner": app.owner.encode_hex::<String>(),
                "homepage": app.homepage
            })
        })
        .collect();

    return Ok(web::Json(mapped));
}
