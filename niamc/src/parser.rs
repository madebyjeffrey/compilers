use common::mapped_file::MappedFile;
use lexer::lexer::Lexer;
use lexer::tokens::Token;
use parser::ast::Program;
use parser::errors::ErrorType;
use parser::parser::Parser;
use parser::token_collection::TokenCollection;

pub fn run_parser(file: &MappedFile, tokens: Vec<Token>) -> Result<Program, ErrorType> {
    println!("Parsing '{}'", file.filename);

    let token_col = TokenCollection::new(tokens);

    let mut parser = Parser::new(token_col, &file.contents);

    let parsed = parser.run();
    
    // still writing...
    // 
    // if lexer.errors.len() > 0 {
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