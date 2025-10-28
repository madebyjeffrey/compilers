use std::collections::HashMap;
use regex::Regex;
use common::span::Span;
use crate::tokens::{char_tokens, identifiers_or_constant, keywords, whitespace, Token, TokenKind};

pub struct Lexer<'a> {
    pub text: &'a str,
    position: usize,
    total_length: usize,
    pub errors: Vec<Span>,
    whitespace: Regex,
    keywords: HashMap<&'static str, TokenKind>,
    id_const: Regex,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'_ str) -> Lexer<'_> {
        Lexer { text,
            position: 0,
            total_length: text.len(),
            errors: Vec::new(),
            whitespace: whitespace(),
            keywords: keywords(),
            id_const: identifiers_or_constant()
        }
    }

    // Appends errors to previous if contiguous
    pub fn add_error_span(&mut self, error: Span) {
        match &mut self.errors.last() {
            None => self.errors.push(error),
            Some(span) => {
                if span.start + span.len == error.start {
                    let mut span = self.errors.pop().unwrap();
                    span.len += error.len;
                    self.errors.push(span);
                } else {
                    self.errors.push(error);
                }
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.position == self.total_length {
                break None
            } else {
                // check whitespace
                if let Some(ws) = self.whitespace.find(&self.text[self.position..]) {
                    self.position += ws.len();
                    continue;
                }

                // check single length symbols
                if let Some(kind) = char_tokens(&self.text[self.position..]) {
                    self.position += 1;
                    break Some(Token { kind, span: Span { start: self.position, len: 1 } });
                }

                if let Some(caps) = self.id_const.captures(&self.text[self.position..]) {
                    // this value must be set as it can't determine on its own that it has been set below
                    let mut span = Span { start: self.position, len: 1 };
                    // this will get overwritten
                    let mut kind = TokenKind::Invalid;

                    if let Some(identifier) = caps.get(1) {
                        span = Span { start: self.position, len: identifier.len() };

                        if let Some(k) = self.keywords.get(&self.text[span.start..][..span.len]) {
                            kind = *k;
                        } else {
                            kind = TokenKind::Identifier;
                        }
                    } else if let Some(constant) = caps.get(2) {
                        span = Span { start: self.position, len: constant.len() };
                        kind = TokenKind::Constant;
                    }

                    self.position += span.len;

                    break Some(Token { kind, span });
                } else {
                    // if we don't have a match add it to the error list
                    let span = Span { start: self.position, len: 1 };
                    self.position += 1;

                    self.add_error_span(span);

                    continue;
                }
            }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn collect_tokens(&mut self) -> Vec<Token> {
        self.by_ref().collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::tokens::TokenKind;
    use super::*;

    #[test]
    fn no_errors1() {
        let mut lexer = Lexer::new("  ");

        let results = lexer.collect_tokens();

        // No errors, but no results either
        assert_eq!(results.len(), 0);
        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("aa a4");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::Identifier);
        assert_eq!(results[0].span.len, 2);

        assert_eq!(results[1].kind, TokenKind::Identifier);
        assert_eq!(results[1].span.len, 2);
    }

    #[test]
    fn at_sign_error() {
        let mut lexer = Lexer::new("return 0@1;");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 4);
        assert_eq!(lexer.errors.len(), 1);
        assert_eq!(results[0].kind, TokenKind::ReturnKeyword);
        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[2].kind, TokenKind::Constant);
        assert_eq!(results[3].kind, TokenKind::Semicolon);
    }

    #[test]
    fn constant() {
        let mut lexer = Lexer::new("0 5");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::Constant);
        assert_eq!(results[0].span.len, 1);

        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[1].span.len, 1);
    }

    #[test]
    fn int_keyword() {
        let mut lexer = Lexer::new("int 5");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::IntKeyword);
        assert_eq!(results[0].span.len, 3);

        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[1].span.len, 1);
    }

    #[test]
    fn void_keyword() {
        let mut lexer = Lexer::new("void 5");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::VoidKeyword);
        assert_eq!(results[0].span.len, 4);

        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[1].span.len, 1);
    }

    #[test]
    fn return_keyword() {
        let mut lexer = Lexer::new("return{");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::ReturnKeyword);
        assert_eq!(results[0].span.len, 6);

        assert_eq!(results[1].kind, TokenKind::OpenBrace);
        assert_eq!(results[1].span.len, 1);
    }

    #[test]
    fn symbols() {
        let mut lexer = Lexer::new("(){};");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 5);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::OpenParen);
        assert_eq!(results[0].span.len, 1);
        assert_eq!(results[1].kind, TokenKind::CloseParen);
        assert_eq!(results[1].span.len, 1);
        assert_eq!(results[2].kind, TokenKind::OpenBrace);
        assert_eq!(results[2].span.len, 1);
        assert_eq!(results[3].kind, TokenKind::CloseBrace);
        assert_eq!(results[3].span.len, 1);
        assert_eq!(results[4].kind, TokenKind::Semicolon);
        assert_eq!(results[4].span.len, 1);
    }
}
