use std::process::ExitCode;
use common::compiler::{Compiler, Phase};
use crate::phases;
use crate::phases::{run_lexer, run_parser};

pub fn compile(main_unit: &str, final_phase: Phase, debug: bool) -> ExitCode {
    let mut compiler = Compiler::new(final_phase, debug)
        .with_lexer(run_lexer)
        .with_parser(run_parser)
        .with_codegen(phases::codegen::codegen::run_codegen);

    if let Err(e) =  compiler.load_main(main_unit) { return e; }

    let Some(tokens) = compiler.lex() else {
        return ExitCode::FAILURE;
    };

    if compiler.should_stop(Phase::Lex) { return ExitCode::SUCCESS; }
    
    let Some(program) = compiler.parse(tokens) else {
        return ExitCode::FAILURE;
    };
    
    if compiler.should_stop(Phase::Parse) { return ExitCode::SUCCESS; }
    
    let irprod = compiler.codegen(program);
    
    if compiler.should_stop(Phase::Codegen) { return ExitCode::SUCCESS; }

    ExitCode::SUCCESS
}