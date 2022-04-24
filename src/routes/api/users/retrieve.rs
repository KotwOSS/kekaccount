use actix_web::{get, web, Responder, Result};

use crate::api::http::State;
use crate::errors::actix::JsonErrorType;
use crate::models::user;
use crate::util::checker::{self, map_qres};

#[get("/api/users/{id}")]
pub async fn retrieve(
    path: web::Path<(String,)>,
    state: web::Data<State>,
) -> Result<impl Responder> {
    let id_hex = path.into_inner().0;
    let id = checker::hex("id", id_hex.as_str(), 32)?;

    let db_connection = &checker::get_con(&state.pool)?;

    let users = map_qres(
        user::User::find(id, db_connection),
        "Error while selecting users",
    )?;

    match users.into_iter().next() {
        Some(user) => Ok(web::Json(json!({
            "name": user.name,
            "avatar": user.avatar,
            "id": id_hex
        }))),
        None => Err(JsonErrorType::NOT_FOUND
            .new_error(format!("User with id '{}' not found!", id_hex))
            .into()),
    }
}
