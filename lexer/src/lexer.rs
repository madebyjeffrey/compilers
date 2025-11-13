use ariadne::Span as ASpan;
use std::collections::HashMap;
use regex::Regex;
use common::span::Span;
use crate::tokens::{char_tokens, identifiers_or_constant, keywords, multiline_comment_start, multiline_comment_start_or_end, newline, single_line_comment_start, whitespace, Token, TokenKind};

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
    keywords: HashMap<&'static str, TokenKind>,
    id_const: Regex,
    multiline_comment_start: Regex,
    single_line_comment_start: Regex,
    multiline_comment_start_or_end: Regex,
    multiline_comment_start_str: &'a str,
    newline: Regex,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'_ str) -> Lexer<'_> {
        Lexer { text,
            position: 0,
            total_length: text.len(),
            stop: false,
            errors: Vec::new(),
            whitespace: whitespace(),
            keywords: keywords(),
            id_const: identifiers_or_constant(),
            multiline_comment_start: multiline_comment_start(),
            multiline_comment_start_or_end: multiline_comment_start_or_end(),
            single_line_comment_start: single_line_comment_start(),
            multiline_comment_start_str: "/*",
            newline: newline(),
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
                    self.position += ws.len();
                    continue;
                }

                if self.position < self.total_length - 2 && let Some(comment_start) = self.multiline_comment_start.find(&self.text[self.position..self.position + 2]) {
                    if let Some(maybe_nested) = self.multiline_comment_start_or_end.find(&self.text[comment_start.end()..]) {
                        // a comment start found within still a comment
                        if maybe_nested.as_str() == self.multiline_comment_start_str {
                            self.errors.push(LexerError::NestedComment(Span::from(comment_start.range()), Span::from(maybe_nested.range())));
                            self.stop = true;
                            break None;
                        } else {
                            self.position = maybe_nested.end() + comment_start.end();
                            continue;
                        }
                    } else {
                        self.errors.push(LexerError::UnexpectedEofInsideComment(Span::from(comment_start.range())));
                        self.stop = true;
                        break None;
                    }
                } // no multiline comment found

                if let Some(comment_start) = self.single_line_comment_start.find(&self.text[self.position..]) {
                    if let Some(eol) = self.newline.find_at(&self.text[self.position..], comment_start.end()) {
                        self.position = self.position + eol.end();
                        continue;
                    } else {
                        self.position = self.total_length;
                        self.stop = true;
                        break None;
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

                        if let Some(k) = self.keywords.get(&self.get_text(&span)) {
                            kind = *k;
                        } else {
                            kind = TokenKind::Identifier;
                        }
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
    use assert_matches::assert_matches;
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
        assert_eq!(results[0].span.range(), 0..2);

        assert_eq!(results[1].kind, TokenKind::Identifier);
        assert_eq!(results[1].span.range(), 3..5);
    }

    #[test]
    fn at_sign_error() {
        let mut lexer = Lexer::new("return 0@1;");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 4);
        assert_eq!(lexer.errors.len(), 1);
        assert_eq!(results[0].kind, TokenKind::ReturnKeyword);
        assert_eq!(results[0].span.range(), 0..6);
        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[1].span.range(), 7..8);
        assert_eq!(results[2].kind, TokenKind::Constant);
        assert_eq!(results[2].span.range(), 9..10);
        assert_eq!(results[3].kind, TokenKind::Semicolon);
        assert_eq!(results[3].span.range(), 11..12);
    }

    #[test]
    fn constant() {
        let mut lexer = Lexer::new("0 5");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::Constant);
        assert_eq!(results[0].span.range(), 0..1);

        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[1].span.range(), 2..3);
    }

    #[test]
    fn int_keyword() {
        let mut lexer = Lexer::new("int 5");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::IntKeyword);
        assert_eq!(results[0].span.range(), 0..3);

        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[1].span.range(), 4..5);
    }

    #[test]
    fn void_keyword() {
        let mut lexer = Lexer::new("void 5");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::VoidKeyword);
        assert_eq!(results[0].span.range(), 0..4);

        assert_eq!(results[1].kind, TokenKind::Constant);
        assert_eq!(results[1].span.range(), 5..6);
    }

    #[test]
    fn return_keyword() {
        let mut lexer = Lexer::new("return{");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 2);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::ReturnKeyword);
        assert_eq!(results[0].span.range(), 0..6);

        assert_eq!(results[1].kind, TokenKind::OpenBrace);
        assert_eq!(results[1].span.range(), 6..7);
    }

    #[test]
    fn symbols() {
        let mut lexer = Lexer::new("(){};");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 5);
        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(results[0].kind, TokenKind::OpenParen);
        assert_eq!(results[0].span.range(), 0..1);
        assert_eq!(results[1].kind, TokenKind::CloseParen);
        assert_eq!(results[1].span.range(), 1..2);
        assert_eq!(results[2].kind, TokenKind::OpenBrace);
        assert_eq!(results[2].span.range(), 2..3);
        assert_eq!(results[3].kind, TokenKind::CloseBrace);
        assert_eq!(results[3].span.range(), 4..5);
        assert_eq!(results[4].kind, TokenKind::Semicolon);
        assert_eq!(results[4].span.range(), 5..6);
    }

    #[test]
    fn real_file1() {
        let mut lexer = Lexer::new("int main(void) {
    return");

        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 7);
        assert_eq!(lexer.errors.len(), 0);
        test_token(&results[0], TokenKind::IntKeyword, 0..3);
        test_token(&results[1], TokenKind::Identifier, 4..8);
        test_token(&results[2], TokenKind::OpenParen, 8..9);
        test_token(&results[3], TokenKind::VoidKeyword, 9..13);
        test_token(&results[4], TokenKind::CloseParen, 13..14);
        test_token(&results[5], TokenKind::OpenBrace, 15..16);
        test_token(&results[6], TokenKind::ReturnKeyword, 21..27);
    }

    #[test]
    fn only_multiline_comment() {
        let mut lexer = Lexer::new("/* hello */");
        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 0);
        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn not_closed_multiline_comment() {
        let mut lexer = Lexer::new("/* hello");
        let results = lexer.collect_tokens();

        assert_eq!(results.len(), 0);
        assert_eq!(lexer.errors.len(), 1);
        assert_matches!(&lexer.errors[..], [LexerError::UnexpectedEofInsideComment(i)] if i.range() == (0..2));
    }

    #[test]
    fn multiline_comment2() {
        let mut lexer = Lexer::new("/* hello */int");
        let results = lexer.collect_tokens();

        assert_matches!(&results[..], [Token { kind: TokenKind::IntKeyword, .. }]);
        assert_matches!(&lexer.errors[..], []);
    }

    #[test]
    fn multiline_comment_with_spaces_after() {
        let mut lexer = Lexer::new("/* hello */          int");
        let results = lexer.collect_tokens();

        assert_matches!(&results[..], [Token { kind: TokenKind::IntKeyword, .. }]);
        assert_matches!(&lexer.errors[..], []);
    }

    #[test]
    fn single_line_comment() {
        let mut lexer = Lexer::new("// hello
        int");

        let results = lexer.collect_tokens();

        assert_matches!(&results[..], [Token { kind: TokenKind::IntKeyword, .. }]);
        assert_matches!(&lexer.errors[..], []);
    }

    #[test]
    fn single_line_comment_will_have_proper_position_after() {
        let mut lexer = Lexer::new(r"// note
new");

        let tokens = lexer.collect_tokens();

        assert_matches!(&tokens[..], [Token { kind: TokenKind::Identifier, span }] if span.range() == (8..11));
        assert_eq!(lexer.position, 11);
    }

    #[test]
    fn does_it_terminate() {
        let mut lexer = Lexer::new(r"
// note: in older versions of C this would be valid
// and return type would default to 'int'
// GCC/Clang will compile it (with a warning)
// for backwards compatibility
main(void) {
    return 0;
}
        ");

        let results = lexer.collect_tokens();

        assert_matches!(&results[..], [
            Token { kind: TokenKind::Identifier, .. },
            Token { kind: TokenKind::OpenParen, .. },
            Token { kind: TokenKind::VoidKeyword, .. },
            Token { kind: TokenKind::CloseParen, .. },
            Token { kind: TokenKind::OpenBrace, .. },
            Token { kind: TokenKind::ReturnKeyword, .. },
            Token { kind: TokenKind::Constant, .. },
            Token { kind: TokenKind::Semicolon, .. },
            Token { kind: TokenKind::CloseBrace, .. },
        ]);
        assert_matches!(&lexer.errors[..], []);
    }

    fn test_token(result: &Token, expected_kind: TokenKind, expected_range: Range<usize>) {
        assert_eq!(result.kind, expected_kind);
        assert_eq!(result.span.range(), expected_range);
    }
}
