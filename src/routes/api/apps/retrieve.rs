use hex::ToHex;

use actix_web::{get, web, Responder, Result};

use crate::api::http::State;
use crate::errors::actix::JsonErrorType;
use crate::models::app;
use crate::util::checker::{self, map_qres};

#[get("/api/apps/{id}")]
pub async fn retrieve(
    path: web::Path<(String,)>,
    state: web::Data<State>,
) -> Result<impl Responder> {
    let id_hex = path.into_inner().0;
    let id = checker::hex("id", id_hex.as_str(), 40)?;

    let db_connection = &checker::get_con(&state.pool)?;

    let apps = map_qres(
        app::App::find(id, db_connection),
        "Error while selecting apps",
    )?;

    match apps.into_iter().next() {
        Some(app) => Ok(web::Json(json!({
            "name": app.name,
            "owner": app.owner.encode_hex::<String>(),
            "homepage": app.homepage,
            "description": app.description,
            "id": id_hex
        }))),
        None => Err(JsonErrorType::NOT_FOUND
            .new_error(format!("App with id '{}' not found!", id_hex))
            .into()),
    }
}
