use lexer::tokens::{Token, TokenKind};
use crate::errors::ErrorType;

#[allow(dead_code)]
pub struct TokenCollection {
    pub tokens: Vec<Token>,
    pub index: usize,
}

impl TokenCollection {
    pub fn new(tokens: Vec<Token>) -> TokenCollection {
        TokenCollection {
            tokens,
            index: 0,
        }
    }

    pub fn take_token(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() {
            let result = Some(&self.tokens[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, ErrorType> {
        let actual = self.take_token();

        if let Some(actual_kind) = actual {
            if actual_kind.kind == kind {
                Ok(*actual_kind)
            } else {
                Err(ErrorType::SyntaxError(*actual_kind, kind))
            }
        } else {
            Err(ErrorType::UnexpectedEOF(kind))
        }
    }
}

#[cfg(test)]
mod tests {
    use common::span::Span;
    use crate::token_collection::ErrorType::{UnexpectedEOF};
    use super::*;

    #[test]
    fn test_tokens() {
        let tokens = vec![Token::new(TokenKind::Constant, Span::new(0, 5))];

        let mut collection = TokenCollection::new(tokens);

        let result1 = collection.expect(TokenKind::Constant);
        let result2 = collection.expect(TokenKind::Constant);

        assert!(result1.is_ok());
        assert_eq!(result2, Err(UnexpectedEOF(TokenKind::Constant)));
    }
}
