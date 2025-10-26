// never in a million c's

use std::process::ExitCode;
use clap::FromArgMatches;
use crate::arguments::{Cli, Mode};
use crate::lexer::run_lexer;
use crate::mapped_file::MappedFile;
use crate::preprocess::preprocess;

mod arguments;
mod lexer;
mod mapped_file;
mod preprocess;

fn main() -> ExitCode {
    let matches = Cli::command().get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap();

    println!("File: {}", cli.file);

    let preprocess = preprocess(&cli.file);

    if let None = preprocess {
        println!("Couldn't preprocess");
        return ExitCode::FAILURE;
    }

    let mapped_file = MappedFile::from_string(&preprocess.unwrap());

    // if let Err(x) = mapped_file {
    //     println!("Error: {}", x);
    //
    //     return ExitCode::FAILURE;
    // }

    let result = match cli.mode {
        Mode::Lexer => {
            println!("Lexing '{}'", cli.file);

            let results = run_lexer(&mapped_file);

            if let Err(x) = results {
                eprintln!("Error lexing:");
                x.iter().for_each(|x| eprintln!("{}", x));
                return ExitCode::FAILURE;
            }

            return ExitCode::SUCCESS;
        },
        Mode::Parse => {
            println!("Lexing, then parsing '{}'", cli.file);
            0
        },
        Mode::CodeGen => {
            println!("Lexing, parsing, then codegen '{}'", cli.file);
            0
        }
    };

    ExitCode::from(result)
}
