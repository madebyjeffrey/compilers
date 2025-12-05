
/*
Instruction <- Ident _ Ident ("," _ Ident)*
Ident       <- [A-Za-z_][A-Za-z0-9_]*
_           <- [ \t]*
 */
use crate::parse_error::ParseError;
use crate::span::Span;

#[derive(Debug)]
struct Operand {
    text: String,
    span: Span,
}

#[derive(Debug)]
struct Instruction {
    opcode: String,
    opcode_span: Span,
    operands: Vec<Operand>,
    span: Span,
}


struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn error(&self, expected: &'static str, msg: impl Into<String>) -> ParseError {
        ParseError::new(self.pos, expected, msg)
    }

    fn consume_whitespace(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t')) {
            self.bump();
        }
    }

    fn parse_ident(&mut self) -> Result<(String, Span), ParseError> {
        let start = self.pos;

        // first character
        let _ = match self.peek() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => c,
            Some(c) => {
                return Err(self.error(
                    "identifier",
                    format!("Unexpected character '{c}' starting identifier"),
                ))
            }
            None => return Err(self.error("identifier", "Unexpected EOF")),
        };
        self.bump();

        // rest of the identifier
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.bump();
            } else {
                break;
            }
        }

        let end = self.pos;
        let text = self.input[start..end].to_string();

        Ok((text, Span { start, end }))
    }

    fn parse_operand_list(&mut self) -> Result<Vec<Operand>, ParseError> {
        let mut ops = Vec::new();

        // first operand
        let (text, span) = self.parse_ident()?;
        ops.push(Operand { text, span });
        self.consume_whitespace();

        // ("," operand)*
        loop {
            match self.peek() {
                Some(',') => {
                    self.bump();
                    self.consume_whitespace();

                    let (text, span) = self.parse_ident()?;
                    ops.push(Operand { text, span });
                    self.consume_whitespace();
                }
                Some(c) if !c.is_whitespace() => {
                    return Err(self.error(
                        "comma",
                        format!("Expected ',', found '{c}'"),
                    ));
                }
                _ => break,
            }
        }

        Ok(ops)
    }

    fn parse_instruction(&mut self) -> Result<Instruction, ParseError> {
        let start = self.pos;
        self.consume_whitespace();

        let (opcode, opcode_span) = self.parse_ident()?;
        self.consume_whitespace();

        let operands = if self.peek().is_some() {
            self.parse_operand_list()?
        } else {
            Vec::new()
        };

        let end = self.pos;

        Ok(Instruction { opcode, opcode_span, operands, span: Span { start, end } })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction() {
        let input = "ADD r1, r2";
        let mut parser = Parser::new(input);

        match parser.parse_instruction() {
            Ok(instr) => {
                assert_eq!(instr.opcode, "ADD");
                assert_eq!(instr.operands[0].text, "r1");
                assert_eq!(instr.operands[1].text, "r2");
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_missing_comma() {
        let input = "ADD r1 r2";
        let mut parser = Parser::new(input);

        match parser.parse_instruction() {
            Ok(instr) => {
                assert!(false);
            },
            Err(err) => assert!(true),
        }
    }
}