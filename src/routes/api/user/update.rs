use actix_web::{post, web, Result, HttpRequest, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::user;
use crate::api::http::State;
use crate::util::checker::{self, map_qres, hex_header};

#[derive(Deserialize)]
pub struct UpdateData {
    name: Option<String>,
    email: Option<String>
}

#[post("/api/user/update")]
pub async fn update(update_data: web::Json<UpdateData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let mut update_count = 0;

    let name: Option<String> = update_data.name.clone();
    if let Some(ref name) = name { 
        checker::min_max_size("Length of name", name.len(), 3, 32)?;
        update_count+=1;
    }

    let email: Option<String> = update_data.email.clone();
    if let Some(ref email) = email { 
        checker::min_max_size("Length of email", email.len(), 3, 32)?;
        checker::email("Email", email.as_str())?;
        update_count+=1;
    }

    if update_count==0 {
        return Err(JsonErrorType::BAD_REQUEST.new_error(format!(
            "You have to add atleast one field which you want to update"
        )).into());
    }

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b1000000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to update yourself. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        map_qres(user::User::update(user.id, &user::UserChangeSet {
            id: None,
            password: None,
            email,
            name
        }, db_connection), "Error while updating user")?;

        return Ok(web::Json(json!({
            "success": true,
            "updated_fields": update_count
        })));
    }
}
