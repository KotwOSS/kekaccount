use std::fmt::{Display, Formatter, self};

pub struct HexError {
    msg: String
}

impl Display for HexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}


pub fn parse_to_buf(hex_string: String, expected_len: usize) -> Result<Vec<u8>, HexError> {
    if hex_string.len() != expected_len {
        return Err(HexError { msg: format!("Expected length {}, but found {}", expected_len, hex_string.len()) })
    }

    hex::decode(hex_string)
        .map_err(|e| HexError { msg: e.to_string() })
}