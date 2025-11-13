use common::span::Span;
use lexer::tokens::{TokenKind};
use crate::ast::{Expression, FunctionDefinition, Program, Statement};
use crate::errors::ParseError;
use crate::token_collection::{TokenCollection};
use crate::utilities::parse_number;

pub struct Parser<'a> {
    tokens: TokenCollection,
    contents: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: TokenCollection, contents: &'a str) -> Parser<'a> {
        Parser { tokens, contents }
    }

    pub fn get_text(&self, span: &Span) -> &'a str {
        &self.contents[span.range()]
    }

    pub fn run(&mut self) -> Result<Program, ParseError> {
        self.parse_program()
    }
}

trait CeeParser {
    fn parse_statement(&mut self) -> Result<Statement, ParseError>;
    fn parse_expression(&mut self) -> Result<Expression, ParseError>;
    fn parse_function(&mut self) -> Result<FunctionDefinition, ParseError>;
    fn parse_program(&mut self) -> Result<Program, ParseError>;
}

impl<'a> CeeParser for Parser<'a> {
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.tokens.expect(TokenKind::ReturnKeyword)?;

        let expr = self.parse_expression()?;

        self.tokens.expect(TokenKind::Semicolon)?;
        
        Ok(Statement::Return(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.tokens.expect(TokenKind::Constant)?;
        let text = self.get_text(&token.span);
        let num = parse_number(text, &token)?;
        Ok(Expression::Constant(num))
    }
    
    fn parse_function(&mut self) -> Result<FunctionDefinition, ParseError> {
        self.tokens.expect(TokenKind::IntKeyword)?;
        let id = self.tokens.expect(TokenKind::Identifier)?;
        let text = self.get_text(&id.span);
        self.tokens.expect(TokenKind::OpenParen)?;
        self.tokens.expect(TokenKind::VoidKeyword)?;
        self.tokens.expect(TokenKind::CloseParen)?;
        self.tokens.expect(TokenKind::OpenBrace)?;
        let statements = self.parse_statement()?;
        self.tokens.expect(TokenKind::CloseBrace)?;
        Ok(FunctionDefinition::Function { name: text.to_string(), body: statements })
    }
    
    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let function_def = self.parse_function()?;

        if self.tokens.is_empty() {
            Ok(Program { function: function_def })
        } else {
            Err(ParseError::ExpectingEOF(self.tokens.last().unwrap().clone()))
        }
    }
}