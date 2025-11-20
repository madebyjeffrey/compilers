// never in a million c's

use std::process::ExitCode;
use argh::FromArgs;
use common::compiler::{Compiler, Phase};
use crate::phases::{run_lexer, run_parser};

mod phases;

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

fn main() -> Result<(), ExitCode> {
    let args: Arguments = argh::from_env();

    let mut final_phase = Phase::FullCompile;

    match (args.lex, args.parse) {
        (false, false) => final_phase = Phase::FullCompile,
        (false, true) => final_phase = Phase::Parse,
        (true, false) => final_phase = Phase::Lex,
        (true, true) => final_phase = Phase::Parse,
    }

    let mut compiler = Compiler::new(final_phase, args.explain)
        .with_lexer(run_lexer)
        .with_parser(run_parser);

    compiler.load_main(&args.input)?;
    let tokens = compiler.lex().ok_or(ExitCode::FAILURE)?;
    compiler.should_stop(Phase::Lex)?;
    let program = compiler.parse(tokens).ok_or(ExitCode::FAILURE)?;
    compiler.should_stop(Phase::Parse)?;
    
    Ok(())
}
