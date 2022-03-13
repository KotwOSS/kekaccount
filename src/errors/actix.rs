use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct JsonError {
    err_type: JsonErrorType,
    msg: String
}

#[derive(Debug)]
pub struct JsonErrorType {
    code: StatusCode,
    msg: &'static str
}


impl JsonErrorType {
    pub const BAD_REQUEST: JsonErrorType = JsonErrorType { code: StatusCode::BAD_REQUEST, msg: "Bad request" };
    pub const MISSING_FIELD: JsonErrorType = JsonErrorType { code: StatusCode::BAD_REQUEST, msg: "Missing field" };
    pub const BAD_CREDENTIALS: JsonErrorType = JsonErrorType { code: StatusCode::UNAUTHORIZED, msg: "Bad credentials" };
    pub const EXISTS: JsonErrorType = JsonErrorType { code: StatusCode::CONFLICT, msg: "Exists" };
    pub const INTERNAL_SERVER_ERROR: JsonErrorType = JsonErrorType { code: StatusCode::INTERNAL_SERVER_ERROR, msg: "Internal server error" };
    pub const DATABASE_ERROR: JsonErrorType = JsonErrorType { code: StatusCode::INTERNAL_SERVER_ERROR, msg: "Database error" };
    pub const NOT_FOUND: JsonErrorType = JsonErrorType { code: StatusCode::NOT_FOUND, msg: "Not found" };
    pub const FORBIDDEN: JsonErrorType = JsonErrorType { code: StatusCode::FORBIDDEN, msg: "Forbidden" };

    pub fn get_code(&self) -> StatusCode {
        self.code
    }

    pub fn get_message(&self) -> &'static str {
        self.msg
    }

    pub fn new_error(self, msg: String) -> JsonError {
        JsonError::new(self, msg)
    }
}

impl JsonError {
    pub fn new(err_type: JsonErrorType, msg: String) -> Self {
        Self { err_type, msg }
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", json!({
            "success": false,
            "status": self.err_type.code.as_u16(),
            "generic": self.err_type.msg,
            "message": self.msg
        }))
    }
}

impl error::ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        self.err_type.code
    }
}