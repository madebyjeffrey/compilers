use std::fmt::Display;
use std::num::{IntErrorKind, ParseIntError};
use lexer::tokens::{Token, TokenKind};

#[derive(Debug, PartialEq, Clone, Eq, Copy)]
#[allow(dead_code)]
pub enum ErrorType {
    SyntaxError(Token, TokenKind),
    UnexpectedEOF(TokenKind),
    InvalidNumber(Token, IntParseError),
}

impl Display for ErrorType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ErrorType::SyntaxError(token, kind) => {
                
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[allow(dead_code)]
pub enum IntParseError {
    Empty,
    /// Contains an invalid digit in its context.
    ///
    /// Among other causes, this variant will be constructed when parsing a string that
    /// contains a non-ASCII char.
    ///
    /// This variant is also constructed when a `+` or `-` is misplaced within a string
    /// either on its own or in the middle of a number.
    InvalidDigit,
    /// Integer is too large to store in target integer type.
    PosOverflow,
    /// Integer is too small to store in target integer type.
    NegOverflow,
    Unknown
}

impl From<ParseIntError> for IntParseError {
    fn from(err: ParseIntError) -> IntParseError {
        match err.kind() {
            IntErrorKind::PosOverflow => IntParseError::PosOverflow,
            IntErrorKind::NegOverflow => IntParseError::NegOverflow,
            IntErrorKind::Empty => IntParseError::Empty,
            IntErrorKind::InvalidDigit => IntParseError::InvalidDigit,
            _ => IntParseError::PosOverflow
        }
    }
}
