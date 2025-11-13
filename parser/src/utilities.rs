use lexer::tokens::Token;
use crate::errors::{ParseError, IntParseError};

pub fn parse_number(text: &str, token: &Token) -> Result<i64, ParseError> {
    match i64::from_str_radix(text, 10) {
        Ok(num) => Ok(num),
        Err(err) => Err(ParseError::InvalidNumber(token.clone(), IntParseError::from(err)))
    }
}