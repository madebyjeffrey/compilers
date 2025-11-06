use common::mapped_file::MappedFile;
use lexer::lexer::Lexer;
use lexer::tokens::Token;
use parser::ast::Program;
use parser::errors::ErrorType;
use parser::parser::Parser;
use parser::token_collection::TokenCollection;

/* T-Dark, Speedrun World Champion — 1:37 AM
In an ideal world, if parsing fails it doesn't consume any tokens so you don't need to backtrack, but in more complex cases backtracking is admittedly required
I usually implement this by parsing in terms of a "cursor" which is just a slice::Iter<'a, Tokens>: clone the cursor (which is cheap: a slice's borrowed iterators just copies two pointers) when you need to possibly backtrack, keep the clone if you do backtrack, keep the original if you don't

 */

pub fn run_parser(file: &MappedFile, tokens: Vec<Token>) -> Result<Program, ErrorType> {
    println!("Parsing '{}'", file.filename);

    let token_col = TokenCollection::new(tokens);

    let mut parser = Parser::new(token_col, &file.contents);

    let parsed = parser.run();

    match parsed {
        Ok(program) => Ok(program),
        Err(error) => {
        }
    }

    if parsed.errors.len() > 0 {
    //     let mut errors: Vec<String> = Vec::new();
    //
    //     for error in lexer.errors {
    //         let location = file.line_pos_from_offset(error.start);
    //
    //         if let Some((line, col)) = location {
    //             let bad_text = &lexer.text[error.start..][..error.len];
    //             let e = format!("Invalid token at line {}, column {}, for {} characters: <{}>", line, col, error.len, bad_text);
    //             errors.push(e);
    //         }
    //     }
    //
    //     return Err(errors);
    // }
    //
    // Ok(tokens)
}