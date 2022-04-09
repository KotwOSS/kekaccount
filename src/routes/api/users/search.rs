use hex::ToHex;
use actix_web::{post, web, Result, Responder};
use serde::Deserialize;

use crate::models::user;
use crate::api::http::State;
use crate::util::checker::{self, map_qres};

#[derive(Deserialize)]
pub struct SearchData {
    name: String,
    exact: Option<bool>,
    offset: Option<i64>,
    limit: Option<i64>
}

#[post("/api/users/search")]
pub async fn search(search_data: web::Json<SearchData>, state: web::Data<State>) -> Result<impl Responder> {
    let name = search_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 0, 32)?;

    let limit = search_data.limit.clone().unwrap_or(10);
    checker::min_max_size("Limit", limit.try_into().unwrap(), 0, 20)?;

    let exact = search_data.exact.unwrap_or(false);

    let offset = search_data.offset.clone().unwrap_or(0);

    let db_connection = &checker::get_con(&state.pool)?;

    let users = map_qres(user::User::ilike_name_ol(
        if exact { name } else { format!("%{}%", name) }, offset, limit, db_connection)
    , "Error while selecting users")?;

    let mapped: Vec<serde_json::Value> = users.into_iter()
        .map(|user| json!({
            "id": user.id.encode_hex::<String>(),
            "avatar": user.avatar,
            "name": user.name
        }))
        .collect();

        return Ok(web::Json(mapped));
}
