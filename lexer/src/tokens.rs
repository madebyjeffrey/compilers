use std::collections::HashMap;
use regex::{Match, Regex};
use crate::span::Span;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Identifier,
    Constant,
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Whitespace,
    Invalid,
}

pub struct TokenDef {
    pub kind: TokenKind,
    pub pattern: Regex
}

pub fn char_tokens(str: &str) -> Option<TokenKind> {
    match str.chars().next().unwrap() {
        '(' => Some(TokenKind::OpenParen),
        ')' => Some(TokenKind::CloseParen),
        '{' => Some(TokenKind::OpenBrace),
        '}' => Some(TokenKind::CloseBrace),
        ';' => Some(TokenKind::Semicolon),
        _ => None
    }
}

pub struct TokenDefResults<'a> {
    pub def: &'a TokenDef,
    pub matcher: Match<'a>
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

pub fn whitespace() -> Regex {
    Regex::new(r"^\s+").unwrap()
}

pub fn keywords() -> HashMap<&'static str, TokenKind> {
    let mut map = HashMap::new();
    map.insert("int", TokenKind::IntKeyword);
    map.insert("void", TokenKind::VoidKeyword);
    map.insert("return", TokenKind::ReturnKeyword);
    map
}

pub fn identifiers_or_constant() -> Regex {
    Regex::new(r"(^[a-zA-Z_][0-9A-Za-z_]*\b)|(^[0-9]+\b)").unwrap()
}