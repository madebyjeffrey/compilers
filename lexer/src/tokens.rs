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
    pub pattern: Regex,
    pub priority: usize,
    pub skip: bool
}

impl TokenDef {
    fn basic(kind: TokenKind, pattern: &str, priority: usize) -> TokenDef {
        TokenDef { kind, pattern: Regex::new(pattern).unwrap(), priority, skip: false }
    }

    fn skip(kind: TokenKind, pattern: &str, priority: usize) -> TokenDef {
        TokenDef { kind, pattern: Regex::new(pattern).unwrap(), priority, skip: true }
    }
}

pub struct TokenDefResults<'a> {
    pub def: &'a TokenDef,
    pub matcher: Match<'a>
}

pub(crate) fn tokens_definitions() -> Vec<TokenDef> {
    vec![
        TokenDef::basic(TokenKind::Identifier, r"^[a-zA-Z_]\w*\b", 10),
        TokenDef::basic(TokenKind::Constant, r"^[0-9]+\b", 10),
        TokenDef::basic(TokenKind::IntKeyword, r"^int\b", 30),
        TokenDef::basic(TokenKind::VoidKeyword, r"^void\b", 30),
        TokenDef::basic(TokenKind::ReturnKeyword, r"^return\b", 30),
        TokenDef::basic(TokenKind::OpenParen, r"^\(", 30),
        TokenDef::basic(TokenKind::CloseParen, r"^\)", 30),
        TokenDef::basic(TokenKind::OpenBrace, r"^\{", 30),
        TokenDef::basic(TokenKind::CloseBrace, r"^}", 30),
        TokenDef::basic(TokenKind::Semicolon, r"^;", 30),
        TokenDef::skip(TokenKind::Whitespace, r"^\s+", 30)
    ]
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

