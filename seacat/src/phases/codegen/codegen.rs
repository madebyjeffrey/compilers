use common::source_file::SourceFile;
use crate::phases::codegen::ir;
use crate::phases::codegen::ir::Program;
use crate::phases::lexical_analysis::tokens::Token;
use crate::phases::parsing::ast;

pub fn run_codegen(program: ast::Program) -> Option<Program> {

}

fn map_program(program: ast::Program) -> Program {

}

fn map_function(function: ast::FunctionDefinition) -> ir::FunctionDefinition {}

fn map_statement(ret: ast::Statement) -> ir::Instructions {}

