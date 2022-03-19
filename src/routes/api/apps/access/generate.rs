use hex::ToHex;
use actix_web::{post, web, Result, Responder};
use serde::Deserialize;

use crate::errors::actix::JsonErrorType;
use crate::models::{user, token, access_code, app};
use crate::api::http::State;
use crate::util::{random, checker::{self, map_qres}};

#[derive(Deserialize)]
pub struct GenerateData {
    username: Option<String>,
    email: Option<String>,
    password: String,
    name: String,
    permissions: i16
}

#[post("/api/apps/{id}/access/generate")]
pub async fn generate(path: web::Path<(String,)>, generate_data: web::Json<GenerateData>, state: web::Data<State>) -> Result<impl Responder> {
    if generate_data.username.is_none() && generate_data.email.is_none() {
        return Err(JsonErrorType::MISSING_FIELD.new_error("Missing username or email!".to_owned()).into());
    }

    let password = checker::hex("password", generate_data.password.as_str(), 128)?;

    let name = generate_data.name.clone();
    checker::min_max_size("Length of name", name.len(), 3, 32)?;

    let app_id_hex = path.into_inner().0;
    let app_id = checker::hex("App id", app_id_hex.as_str(), 40)?;

    let db_connection = &checker::get_con(&state.pool)?;
    
    let users = match generate_data.username.clone() {
        Some(username) => map_qres(user::User::find_name(username, password, db_connection), "Error while selecting users"),
        None => map_qres(user::User::find_email(generate_data.email.clone().unwrap(), password, db_connection), "Error while selecting users")
    }?;

    match users.into_iter().next() {
        Some(user) => {
            let apps = map_qres(app::App::find(app_id, db_connection), "Error while selecting apps")?;

            match apps.into_iter().next() {
                Some(app) => {
                    let access_code = random::random_byte_array(32);
                    let access_code_hex = access_code.encode_hex::<String>();

                    let token = random::random_byte_array(128);

                    let id = random::random_byte_array(8);
                    let id_hex = id.encode_hex::<String>();

                    let new_access_code = access_code::AccessCode { id: access_code, token_id: id.clone(), app_id: app.id };
                    map_qres(new_access_code.create(db_connection), "Error while inserting access code")?;

                    let new_token = token::Token { id, token, name, user_id: user.id, permissions: generate_data.permissions };
                    map_qres(new_token.create(db_connection), "Error while inserting token")?;

                    Ok(web::Json(json!({
                        "success": true,
                        "code": access_code_hex,
                        "id": id_hex
                    })))
                },
                None => Err(JsonErrorType::NOT_FOUND.new_error(format!(
                    "App with id '{}' not found!",
                    app_id_hex
                )).into())
            }
            
        },
        None => Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
            "Invalid {} and or password!", 
            if generate_data.username.is_some() {"username"} else {"email"}
        )).into())
    }
}
