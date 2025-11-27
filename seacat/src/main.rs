// never in a million c's

use std::process::ExitCode;
use argh::FromArgs;
use common::compiler::{Compiler, Phase};
use crate::compiler::compile;
use crate::phases::{run_lexer, run_parser};

mod phases;
mod compiler;

#[derive(FromArgs)]
#[argh(description = "Minimal C Compiler")]
pub struct Arguments {
    #[argh(switch, short = 'e', description = "enable debug logging")]
    pub explain: bool,

    #[argh(switch, description = "lex the input file")]
    pub lex: bool,

    #[argh(switch, description = "parse the input file")]
    pub parse: bool,

    #[argh(switch, description = "run codegen", long = "codegen")]
    pub codegen: bool,

    #[argh(positional, description = "the file to read")]
    pub input: String,
}

fn main() -> ExitCode {
    let args: Arguments = argh::from_env();

    let mut final_phase = Phase::FullCompile;

    match (args.lex, args.parse, args.codegen) {
        (false, false, false) => final_phase = Phase::FullCompile,
        (false, true, false) => final_phase = Phase::Parse,
        (true, false, false) => final_phase = Phase::Lex,
        (true, true, false) => final_phase = Phase::Parse,
        (false, false, true) => final_phase = Phase::Codegen,
        (false, true, true) => final_phase = Phase::Codegen,
        (true, false, true) => final_phase = Phase::Codegen,
        (true, true, true) => final_phase = Phase::Codegen,
    }

    compile(&args.input, final_phase, args.explain)
}