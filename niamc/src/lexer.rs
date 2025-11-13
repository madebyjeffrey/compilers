use ariadne::{Color, Label, Report, ReportKind};
use common::source_file::SourceFile;
use lexer::lexer::{Lexer, LexerError};
use lexer::tokens::Token;

pub fn run_lexer(file: &SourceFile, explain: bool) -> Option<Vec<Token>> {
    println!("Lexing '{}'", file.filename);

    let mut lexer = Lexer::new(&file.source.text());
    let (tokens, errors) = lexer.run();

    if tokens.len() > 0 && explain {
       for token in &tokens {
           println!("{}", token.explain(&file.source.text()));
       }
    }

    if errors.len() > 0 {
        for error in lexer.errors {
            match error {
                LexerError::UnexpectedEofInsideComment(span) => {
                    Report::build(ReportKind::Error, span.clone())
                        .with_code("L001")
                        .with_message("Comment started but not finished before end of file.")
                        .with_label(Label::new(span)
                            .with_message("Comment starts here")
                            .with_color(Color::Primary))
                        .finish()
                        .eprint(file)
                        .unwrap();
                },
                LexerError::NestedComment(comment_start, nested_comment_start) => {
                    Report::build(ReportKind::Error, comment_start.clone())
                        .with_code("L002")
                        .with_message("Unknown token.")
                        .with_label(Label::new(comment_start)
                            .with_message("Comment starts here")
                            .with_color(Color::Primary))
                        .with_label(Label::new(nested_comment_start)
                            .with_message("Nested comment starts here")
                            .with_color(Color::Primary))
                        .finish()
                        .eprint(file)
                        .unwrap();
                },
                LexerError::UnknownToken(span) => {
                    Report::build(ReportKind::Error, span.clone())
                        .with_code("L003")
                        .with_message("Unknown token.")
                        .with_label(Label::new(span)
                            .with_message("Unknown token")
                            .with_color(Color::Primary))
                        .finish()
                        .eprint(file)
                        .unwrap();
                },
            }
        }

        return None;
    }

    Some(tokens)
}