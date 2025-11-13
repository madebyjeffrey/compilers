// never in a million c's

use std::process::ExitCode;
use argh::FromArgs;
use common::source_file::{Id, SourceFile};
use crate::lexer::run_lexer;
use crate::parser::run_parser;

mod lexer;
mod parser;

#[derive(FromArgs)]
#[argh(description = "Minimal C Compiler")]
pub struct Arguments {
    #[argh(switch, short = 'e', description = "enable debug logging")]
    pub explain: bool,

    #[argh(switch, description = "lex the input file")]
    pub lex: bool,

    #[argh(switch, description = "parse the input file")]
    pub parse: bool,

    #[argh(positional, description = "the file to read")]
    pub input: String,
}

fn main() -> ExitCode {
    let mut args: Arguments = argh::from_env();

    if !args.lex && !args.parse {
        args.parse = true;
    }

    if args.lex && args.parse {
        args.lex = false;
    }

    if args.lex {
        let mut main = match SourceFile::from_file(Id::Main, &args.input) {
            Err(err) => {
                println!("Couldn't read file: {}", err);
                return ExitCode::FAILURE;
            },
            Ok(main) => main,
        };

        return match run_lexer(&mut main, args.explain) {
            Some(_) => {
                println!("Lexer lexed successfully");
                ExitCode::SUCCESS
            },
            None => ExitCode::FAILURE
        }
    }

    if args.parse {
        let mut main = match SourceFile::from_file(Id::Main, &args.input) {
            Err(err) => {
                println!("Couldn't read file: {}", err);
                return ExitCode::FAILURE;
            },
            Ok(main) => main,
        };

        return match run_lexer(&mut main, args.explain) {
            Some(tokens) => {
                println!("Lexer lexed successfully");

                if let Some(_) = run_parser(&mut main, tokens, args.explain) {
                    println!("Parsed successfully");
                    return ExitCode::SUCCESS;
                }

                ExitCode::FAILURE
            },
            None => ExitCode::FAILURE
        }
    }

    ExitCode::SUCCESS
}
