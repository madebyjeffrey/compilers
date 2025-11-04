use lexer::tokens::Token;
use crate::errors::{ErrorType, IntParseError};

pub fn parse_number(text: &str, token: &Token) -> Result<i64, ErrorType> {
    match i64::from_str_radix(text, 10) {
        Ok(num) => Ok(num),
        Err(err) => Err(ErrorType::InvalidNumber(*token, IntParseError::from(err)))
    }
}