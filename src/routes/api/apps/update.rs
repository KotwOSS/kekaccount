use actix_web::{post, web, Result, HttpRequest, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::app;
use crate::api::http::State;
use crate::util::checker::{self, map_qres, hex_header};

#[derive(Deserialize)]
pub struct UpdateData {
    id: String,
    name: Option<String>,
    avatar: Option<String>,
    description: Option<String>,
    redirect_uri: Option<String>,
    homepage: Option<String>,
}

#[post("/api/apps/update")]
pub async fn update(update_data: web::Json<UpdateData>, state: web::Data<State>, request: HttpRequest) -> Result<impl Responder> {
    let token = hex_header("Authorization", 256, request.headers())?;

    let id = checker::hex("id", update_data.id.as_str(), 40)?;

    let mut update_count = 0;

    let name: Option<String> = update_data.name.clone();
    if let Some(ref name) = name { 
        checker::min_max_size("Length of name", name.len(), 3, 32)?; 
        update_count+=1;
    }

    let avatar: Option<String> = update_data.avatar.clone();
    if let Some(ref avatar) = avatar { 
        checker::min_max_size("Length of avatar", avatar.len(), 0, 255)?;
        update_count+=1;
    }

    let description: Option<String> = update_data.description.clone();
    if let Some(ref description) = description { 
        checker::min_max_size("Length of description", description.len(), 0, 255)?;
        update_count+=1;
    }

    let redirect_uri: Option<String> = update_data.redirect_uri.clone();
    if let Some(ref redirect_uri) = redirect_uri { 
        checker::min_max_size("Length of redirect_uri", redirect_uri.len(), 0, 255)?;
        update_count+=1;
    }

    let homepage: Option<String> = update_data.homepage.clone();
    if let Some(ref homepage) = homepage { 
        checker::min_max_size("Length of homepage", homepage.len(), 0, 255)?;
        update_count+=1;
    }

    if update_count==0 {
        return Err(JsonErrorType::BAD_REQUEST.new_error(format!(
            "You have to add atleast one field which you want to update"
        )).into());
    }

    let db_connection = &checker::get_con(&state.pool)?;

    let (user, token) = checker::authorize(token, db_connection)?;

    if token.permissions & 0b1000 == 0 {
        return Err(JsonErrorType::FORBIDDEN.new_error(format!(
            "You don't have the permissions to update apps. (Your permission level: {})",
            token.permissions
        )).into());
    } else {
        return match map_qres(app::App::update_owner(id, user.id, &app::AppChangeSet {
            id: None,
            owner: None,
            name,
            avatar,
            description: update_data.description.clone(),
            redirect_uri: update_data.redirect_uri.clone(),
            homepage: update_data.homepage.clone(),
        }, db_connection), "Error while updating app")? {
            0 => Err(JsonErrorType::NOT_FOUND.new_error(format!(
                "App with id '{}' not found!",
                update_data.id
            )).into()),
            _ => Ok(web::Json(json!({
                "success": true,
                "updated_fields": update_count
            })))
        };
    }
}
