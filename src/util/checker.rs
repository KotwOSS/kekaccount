use regex::{self, Regex};
use tokio::sync::OnceCell;
use crate::{errors::actix::{JsonError, JsonErrorType}, colors};


#[derive(Debug)]
pub struct Logger {
    // ...
}
static EMAIL_REGEX: OnceCell<Regex> = OnceCell::const_new();

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
    //
}

pub fn init() {
    println!(
        "{}INIT{} checker.rs",
        colors::GREEN,
        colors::RESET
    );
    
    EMAIL_REGEX.set(Regex::new(r"^\w+[\+\.\w-]*@([\w-]+\.)*\w+[\w-]*\.([a-z]{2,4}|\d+)$").unwrap()).unwrap();
}