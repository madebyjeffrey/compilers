use std::collections::HashMap;
use regex::Regex;
use crate::span::Span;
use crate::tokens::{char_tokens, keywords, whitespace, Token, TokenDef, TokenDefResults, TokenKind};

pub struct Lexer<'a> {
    pub text: &'a str,
    position: usize,
    total_length: usize,
    matchers: Vec<TokenDef>,
    pub errors: Vec<Span>,
    whitespace: Regex,
    keywords: HashMap<&'static str, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'_ str) -> Lexer<'_> {
        Lexer { text,
            position: 0,
            total_length: text.len(),
            matchers: crate::tokens::tokens_definitions(),
            errors: Vec::new(),
            whitespace: whitespace(),
            keywords: keywords()
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

                // Check all regexes against the string
                let mut results: Vec<TokenDefResults> = self.matchers.iter()
                    .map(|e| (e, e.pattern.find(&self.text[self.position..])))
                    .filter(|e| e.1.is_some())
                    .map(|e| TokenDefResults { def: e.0, matcher: e.1.unwrap() })
                    .collect();

                // sort descending first by length, and then by priority
                results.sort_by(
                    |a, b| b.matcher.len().cmp(&a.matcher.len()));


                // first one is the longest, highest priority match
                let result = results.first();

                match result {
                    // if we have a match
                    Some(result) => {
                        let span = Span { start: self.position, len: result.matcher.len() };
                        self.position += span.len;

                        let mut kind = result.def.kind;

                        if kind == TokenKind::Identifier &&
                            let Some(k) = self.keywords.get(&self.text[span.start..][..span.len]) {
                            kind = *k;
                        }

                        break Some(Token { kind, span });
                    },
                    None => {
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
