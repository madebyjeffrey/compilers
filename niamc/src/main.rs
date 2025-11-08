// never in a million c's

use std::process::ExitCode;
use clap::FromArgMatches;
use codespan_reporting::files::SimpleFile;
use crate::arguments::{Cli, Mode};
use crate::lexer::run_lexer;
use crate::preprocess::preprocess;
use common::mapped_file::*;
use crate::files::{file_contents, load_file};

mod arguments;
mod lexer;
mod preprocess;
mod parser;
mod files;

fn main() -> ExitCode {
    let matches = Cli::command().get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap();

    println!("File: {}", cli.filename);

    let preprocess = preprocess(&cli.filename);

    if let None = preprocess {
        println!("Couldn't preprocess");
        return ExitCode::FAILURE;
    }

    let sfile = match load_file(&cli.filename) {
        Err(err) => {
            println!("Couldn't read file: {}", err);
            return ExitCode::FAILURE;
        },
        Ok(sfile) => sfile,
    };

    let result = match cli.mode {
        Mode::Lexer => {
            println!("Lexing '{}'", cli.filename);

            let results = run_lexer(&mapped_file);

            if let Err(x) = results {
                eprintln!("Error lexing:");
                x.iter().for_each(|x| eprintln!("{}", x));
                return ExitCode::FAILURE;
            }

            return ExitCode::SUCCESS;
        },
        Mode::Parse => {
            println!("Lexing, then parsing '{}'", cli.filename);
            0
        },
        Mode::CodeGen => {
            println!("Lexing, parsing, then codegen '{}'", cli.filename);
            0
        }
    };

    ExitCode::from(result)
}
