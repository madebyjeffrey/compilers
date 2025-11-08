use ariadne::Source;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::Files;
use lexer::lexer::Lexer;
use lexer::tokens::Token;

pub fn run_lexer(file: &Source) -> Result<Vec<Token>, Vec<String>> {
    println!("Lexing '{}'", file.name());

    let mut lexer = Lexer::new(&file.text());
    let tokens: Vec<Token> = lexer.collect_tokens();

    if lexer.errors.len() > 0 {
        let mut errors: Vec<Diagnostic<String>> = Vec::new();

        for error in lexer.errors {
            match file.location((), error.start) {
                // Handle errors that shouldn't happen
                Err(e) => {
                    errors.push(Diagnostic::error()
                        .with_message("Internal error during lexing.")
                        .with_note(format_args!("Notes for investigation: {}", e)));
                },
            }
        }

        if let Some((line, col)) = location {
                let bad_text = &lexer.text[error.start..][..error.len];
                let e = format!("Invalid token at line {}, column {}, for {} characters: <{}>", line, col, error.len, bad_text);
                errors.push(e);
            }
        }

        return Err(errors);
    }

    Ok(tokens)
}