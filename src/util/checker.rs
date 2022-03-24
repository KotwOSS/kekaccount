use actix_web::http::header::HeaderMap;
use diesel::{QueryResult, PgConnection, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use regex::{self, Regex};
use tokio::sync::OnceCell;
use crate::{errors::actix::{JsonError, JsonErrorType}, util, colors, models::{user, token, app_token, app}, database::PgPool};

static EMAIL_REGEX: OnceCell<Regex> = OnceCell::const_new();
//static URL_REGEX: OnceCell<Regex> = OnceCell::const_new();

pub fn min_max_size<'a>(name: &'a str, len: usize, min: usize, max: usize) -> Result<(), JsonError> {
    if len < min || len > max {
        return Err(JsonErrorType::BAD_REQUEST.new_error(format!(
            "{} must be in bounds {}-{}!",
            name, min, max
        )));
    }
    Ok(())
}

pub fn email<'a, 'b>(name: &'a str, email: &'b str) -> Result<(), JsonError> {
    let regex = EMAIL_REGEX.get().unwrap();
    if !regex.is_match(email) {
        return Err(JsonErrorType::BAD_REQUEST.new_error(format!(
            "{} must be a valid email address!",
            name
        )));
    }
    Ok(())
}

pub fn init() {
    println!(
        "{}INIT{} checker.rs",
        colors::GREEN,
        colors::RESET
    );

    EMAIL_REGEX.set(Regex::new(r"^\w+[\+\.\w-]*@([\w-]+\.)*\w+[\w-]*\.([a-z]{2,18}|\d+)$").unwrap()).unwrap();
    // TODO: write a regex that parses
    //URL_REGEX.set(Regex::new(r"^(ht|f)tps?:\/\/(localhost|((\w+-+)*\w+\.){1,}[a-zA-Z]{2,18}|(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3})(:(4915[01]|491[0-4][0-9]|490[0-9]{2}|4[0-8][0-9]{3}|[0-3]?[0-9]?[0-9]?[0-9]?[0-9]))?(\/([\w%\- ]+\/?)*(\?[\w-]+=[\w-]+(&[\w-]+=[\w-]+)*)?)?$").unwrap()).unwrap();
}

pub fn map_qres<'a, T>(qres: QueryResult<T>, text: &'a str) -> Result<T, JsonError> {
    qres.map_err(|e| JsonErrorType::DATABASE_ERROR.new_error(format!(
        "{}: {}",
        text,
        e
    )))
}

pub fn map_opt<'a, T>(opt: Option<T>, msg: &'a str) -> Result<T, JsonError> {
    match opt {
        Some(val) => Ok(val),
        None => Err(JsonErrorType::MISSING_FIELD.new_error(format!(
            "{}",
            msg
        )))
    }
}

pub fn get_con(pool: &PgPool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, JsonError> {
    pool.get().map_err(|e| JsonErrorType::DATABASE_ERROR.new_error(format!(
        "Database unreachable! Please contact an administratior. ({})",
        e
    )))
}

pub fn hex_header<'a>(name: &'a str, len: usize, headers: &HeaderMap)->Result<Vec<u8>, JsonError> {
    hex(name, map_opt(headers.get(name), format!("Missing {} header", name).as_str())?.to_str().unwrap(), len)
}

pub fn hex<'a, 'b>(name: &'a str, hex: &'b str, len: usize)->Result<Vec<u8>, JsonError> {
    util::hex::parse_to_buf(hex, len)
        .map_err(|e| JsonErrorType::BAD_REQUEST.new_error(format!(
            "Error while parsing {}: {}",
            name,
            e
        )))
}

pub fn authorize(token: Vec<u8>, db_connection: &PgConnection) -> Result<(user::User, token::Token), JsonError> {
    let tokens = map_qres(token::Token::find_token(token, db_connection), "Error while selecting tokens")?;

    match tokens.into_iter().next() {
        Some(token) => {
            let users = map_qres(user::User::find(token.user_id.clone(), db_connection), "Error while selecting users")?;

            match users.into_iter().next() {
                None=>Err(JsonErrorType::NOT_FOUND.new_error(format!(
                    "The account behind this token doesn't exist anymore. Has it been deleted?"
                ))),
                Some(user)=>Ok((user, token))
            }
        },
        None => Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
            "Invalid token!"
        )))
    }
}

pub fn authorize_token(token: Vec<u8>, db_connection: &PgConnection) -> Result<token::Token, JsonError> {
    let tokens = map_qres(token::Token::find_token(token, db_connection), "Error while selecting tokens")?;

    match tokens.into_iter().next() {
        Some(token) => Ok(token),
        None => Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
            "Invalid token!"
        )))
    }
}

pub fn authorize_app(token: Vec<u8>, db_connection: &PgConnection) -> Result<(app::App, app_token::AppToken), JsonError> {
    let tokens = map_qres(app_token::AppToken::find_token(token, db_connection), "Error while selecting tokens")?;

    match tokens.into_iter().next() {
        Some(token) => {
            let apps = map_qres(app::App::find(token.app_id.clone(), db_connection), "Error while selecting users")?;

            match apps.into_iter().next() {
                None=>Err(JsonErrorType::NOT_FOUND.new_error(format!(
                    "The app behind this token doesn't exist anymore. Has it been deleted?"
                ))),
                Some(app)=>Ok((app, token))
            }
        },
        None => Err(JsonErrorType::BAD_CREDENTIALS.new_error(format!(
            "Invalid token!"
        )))
    }
}