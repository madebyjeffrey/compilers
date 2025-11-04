use common::mapped_file::MappedFile;
use lexer::lexer::Lexer;
use lexer::tokens::Token;

pub fn run_lexer(file: &MappedFile) -> Result<Vec<Token>, Vec<String>> {
    println!("Lexing '{}'", file.filename);

    let mut lexer = Lexer::new(&file.contents);
    let tokens: Vec<Token> = lexer.collect_tokens();

    if lexer.errors.len() > 0 {
        let mut errors: Vec<String> = Vec::new();

        for error in lexer.errors {
            let location = file.line_pos_from_offset(error.start);

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