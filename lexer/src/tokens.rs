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
    Whitespace
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

impl TokenDef {
    fn basic(kind: TokenKind, pattern: &str) -> TokenDef {
        TokenDef { kind, pattern: Regex::new(pattern).unwrap() }
    }
}

pub struct TokenDefResults<'a> {
    pub def: &'a TokenDef,
    pub matcher: Match<'a>
}

pub(crate) fn tokens_definitions() -> Vec<TokenDef> {
    vec![
        TokenDef::basic(TokenKind::Identifier, r"^[a-zA-Z_][0-9A-Za-z_]*\b"),
        TokenDef::basic(TokenKind::Constant, r"^[0-9]+\b"),
    ]
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