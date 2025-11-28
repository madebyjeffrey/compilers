use std::collections::HashMap;
use std::fmt::{Debug, Display};
use regex::{Regex};
use common::span::Span;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenKind {
    Identifier,
    Colon,
    FullStop,
    Constant,
    LineFeed,
    Whitespace,
    Invalid,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
    pub fn explain(&self, source: &str) -> String {
        format!("[{}]:{} text: \"{}\"", self.kind, self.span, &source[self.span.range()])
    }
}

pub fn char_tokens(str: &str) -> Option<TokenKind> {
    match str.chars().next().unwrap() {
        '.' => Some(TokenKind::FullStop),
        ':' => Some(TokenKind::Colon),
        _ => None
    }
}

pub fn whitespace() -> Regex {
    Regex::new(r"^[^\S\r\n]+").unwrap()
}

pub fn identifiers_or_constant() -> Regex {
    Regex::new(r"(^[a-zA-Z_][0-9A-Za-z_]*\b)|(^[0-9]+\b)").unwrap()
}

pub fn single_line_comment_start() -> Regex {
    Regex::new(r"^//").unwrap()
}

pub fn newline() -> Regex {
    Regex::new(r"\n|\r\n").unwrap()
}

pub fn newline_only_here() -> Regex {
    Regex::new(r"^\n|\r\n").unwrap()
}