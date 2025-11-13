use std::collections::HashMap;
use std::fmt::{Debug, Display};
use regex::{Match, Regex};
use common::span::Span;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
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

pub fn multiline_comment_start() -> Regex {
    Regex::new(r"^/\*").unwrap()
}

pub fn multiline_comment_end() -> Regex {
    Regex::new(r"\*/").unwrap()
}

pub fn multiline_comment_start_or_end() -> Regex {
    Regex::new(r"/\*|\*/").unwrap()
}

pub fn single_line_comment_start() -> Regex {
    Regex::new(r"^//").unwrap()
}

pub fn newline() -> Regex {
    Regex::new(r"\n|\r\n").unwrap()
}
