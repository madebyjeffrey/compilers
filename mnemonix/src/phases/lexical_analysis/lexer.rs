use ariadne::Span as ASpan;
use regex::Regex;
use common::span::Span;
use crate::phases::lexical_analysis::tokens::{char_tokens, identifiers_or_constant, newline, newline_only_here, whitespace, Token, TokenKind};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LexerError {
    UnknownToken(Span),
    NestedComment(Span, Span), // start, second start found
    UnexpectedEofInsideComment(Span) // where started
}

pub struct Lexer<'a> {
    pub text: &'a str,
    position: usize,
    total_length: usize,
    stop: bool,
    pub errors: Vec<LexerError>,
    whitespace: Regex,
    id_const: Regex,
    newline: Regex,
    newline_only_here: Regex,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'_ str) -> Lexer<'_> {
        Lexer { text,
            position: 0,
            total_length: text.len(),
            stop: false,
            errors: Vec::new(),
            whitespace: whitespace(),
            id_const: identifiers_or_constant(),
            newline: newline(),
            newline_only_here: newline_only_here(),
        }
    }

    // Appends errors to previous if contiguous
    pub fn add_error_span(&mut self, error: Span) {
        if let Some(LexerError::UnknownToken(span)) = self.errors.last_mut() && span.end() == error.start() {
            span.expand(error.len());
        } else {
            self.errors.push(LexerError::UnknownToken(error));
        }
    }

    fn collect_tokens(&mut self) -> Vec<Token> {
        self.by_ref().collect()
    }

    pub fn run(&mut self) -> (Vec<Token>, Vec<LexerError>) {
        let tokens = self.collect_tokens();

        (tokens, self.errors.clone())
    }

    pub fn get_text(&self, span: &Span) -> &'a str {
        &self.text[span.range()]
    }

    pub fn next_is(&self, text: &str) -> Option<Span> {
        let start = self.position;

        let n = (self.total_length - self.position).min(text.len());

        if n < text.len() {
            return None;
        }

        if &self.text[self.position..self.position + n] == text {
            return Some(Span::new(self.position, n));
        }
        
        return None;
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.position == self.total_length || self.stop {
                break None
            } else {
                // check whitespace
                if let Some(ws) = self.whitespace.find(&self.text[self.position..]) {
                    let start = self.position;
                    self.position += ws.len();
                    break Some(Token { kind: TokenKind::Whitespace, span: Span::new(start, ws.len()) });
                }
                
                // check newline
                if let Some(eol) = self.next_is("\n").or(self.next_is("\r\n")) {
                    let start = self.position;
                    self.position += eol.len();

                    break Some(Token { kind: TokenKind::LineFeed, span: Span::new(start, eol.len()) });
                }

                // check for comment ';'
                if self.text[self.position..].starts_with(';') {
                    if let Some(eol) = self.newline.find(&self.text[self.position..]) {
                        self.position += eol.end();
                        
                        let eol_start = self.position - eol.len();

                        break Some(Token { kind: TokenKind::LineFeed, span: Span::new(eol_start, eol.len()) });
                    }
                }
                
                // check single length symbols
                if let Some(kind) = char_tokens(&self.text[self.position..]) {
                    self.position += 1;
                    break Some(Token { kind, span: Span::new(self.position-1, 1) });
                }

                if let Some(caps) = self.id_const.captures(&self.text[self.position..]) {
                    // this value must be set as it can't determine on its own that it has been set below
                    let mut span = Span::new(self.position, 1);
                    // this will get overwritten
                    let mut kind = TokenKind::Invalid;

                    if let Some(identifier) = caps.get(1) {
                        span = Span::new(self.position, identifier.len());
                        kind = TokenKind::Identifier;
                    } else if let Some(constant) = caps.get(2) {
                        span = Span::new(self.position, constant.len());
                        kind = TokenKind::Constant;
                    }

                    self.position += span.len();

                    break Some(Token { kind, span });
                } else {
                    // if we don't have a match add it to the error list
                    let span = Span::new(self.position, 1);
                    self.position += 1;

                    self.add_error_span(span);

                    continue;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Range;
    use crate::phases::lexical_analysis::tokens::TokenKind;
    use super::*;

    #[test]
    fn spaces() {
        let mut lexer = Lexer::new("  ");

        let results = lexer.collect_tokens();

        // No errors, but no results either
        assert_eq!(results.len(), 1);
        assert_eq!(lexer.errors.len(), 0);
        test_token(&results[0], TokenKind::Whitespace, 0..2);
    }

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("aa a4");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 3);
        assert_eq!(lexer.errors.len(), 0);
        test_token(&results[0], TokenKind::Identifier, 0..2);
        test_token(&results[1], TokenKind::Whitespace, 2..3);
        test_token(&results[2], TokenKind::Identifier, 3..5);
    }

    #[test]
    fn general_test() {
        let mut lexer = Lexer::new("Label:                          ; A label and a comment\r\n");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 4);
        assert_eq!(lexer.errors.len(), 0);
        test_token(&results[0], TokenKind::Identifier, 0..5);
        test_token(&results[1], TokenKind::Colon, 5..6);
        test_just_token(&results[2], TokenKind::Whitespace);
        test_just_token(&results[3], TokenKind::LineFeed);
    }

    #[test]
    fn constant() {
        let mut lexer = Lexer::new("0 5");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 3);
        assert_eq!(lexer.errors.len(), 0);
        test_token(&results[0], TokenKind::Constant, 0..1);
        test_just_token(&results[1], TokenKind::Whitespace);
        test_token(&results[2], TokenKind::Constant, 2..3);
    }

    #[test]
    fn full_stop() {
        let mut lexer = Lexer::new(".");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 1);
        assert_eq!(lexer.errors.len(), 0);
        test_token(&results[0], TokenKind::FullStop, 0..1);
    }

    fn test_token(result: &Token, expected_kind: TokenKind, expected_range: Range<usize>) {
        assert_eq!(result.kind, expected_kind);
        assert_eq!(result.span.range(), expected_range);
    }

    fn test_just_token(result: &Token, expected_kind: TokenKind) {
        assert_eq!(result.kind, expected_kind);
    }
}
