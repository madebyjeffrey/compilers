use ariadne::{Color, Label, Report, ReportKind};
use common::source_file::SourceFile;
use lexer::tokens::Token;
use parser::ast::Program;
use parser::errors::ParseError;
use parser::parser::Parser;
use parser::token_collection::TokenCollection;

/* T-Dark, Speedrun World Champion — 1:37 AM
In an ideal world, if parsing fails it doesn't consume any tokens so you don't need to backtrack, but in more complex cases backtracking is admittedly required
I usually implement this by parsing in terms of a "cursor" which is just a slice::Iter<'a, Tokens>: clone the cursor (which is cheap: a slice's borrowed iterators just copies two pointers) when you need to possibly backtrack, keep the clone if you do backtrack, keep the original if you don't

 */

#[allow(unused)]
pub fn run_parser(file: &SourceFile, tokens: Vec<Token>, explain: bool) -> Option<Program> {
    println!("Parsing '{}'", file.filename);

    let token_col = TokenCollection::new(tokens);

    let mut parser = Parser::new(token_col, &file.source.text());

    let parsed = parser.run();

    match parsed {
        Ok(program) => {
            if explain {
                println!("{:#?}", program);
            }

            Some(program)
        }
        Err(ParseError::UnexpectedEOF(token)) => {
            let _  = Report::build(ReportKind::Error, file.eof())
                .with_message(format!("Unexpected end of file, expected token '{:?}'", token))
                .finish()
                .print(file);
            None
        },
        Err(ParseError::SyntaxError(found, wanted)) => {
            let _ = Report::build(ReportKind::Error, found.span)
                .with_message(format!("Syntax error. Expected token '{:?}'", wanted))
                .finish()
                .print(file);
            None
        },
        Err(ParseError::InvalidNumber(found, error)) => {
            let _ = Report::build(ReportKind::Error, found.span)
                .with_message(format!("Invalid constant: '{:?}'", error))
                .finish()
                .print(file);
            None
        },
        Err(ParseError::ExpectingEOF(token)) => {
            let _ = Report::build(ReportKind::Error, token.span.clone())
                .with_message(format!("Expecting EOF"))
                .with_label(Label::new(token.span)
                    .with_message("Unexpected token here.")
                    .with_color(Color::Primary))
                .finish()
                .print(file);
            None
        }
    }
}