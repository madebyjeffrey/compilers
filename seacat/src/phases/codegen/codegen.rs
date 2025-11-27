use common::source_file::SourceFile;
use crate::phases::codegen::ir;
use crate::phases::codegen::ir::Program;
use crate::phases::lexical_analysis::tokens::Token;
use crate::phases::parsing::ast;

pub fn run_codegen(program: ast::Program) -> ir::Program {
    map_program(program)
}

fn map_program(program: ast::Program) -> ir::Program {
    let ast::Program { function } = program;
    
    Program { function: map_function(function) }
}

fn map_function(function: ast::FunctionDefinition) -> ir::FunctionDefinition {
    let ast::FunctionDefinition::Function { name, body } = function;
    
    let instructions = map_statement(body);
    
    ir::FunctionDefinition { name, instructions }
}

fn map_statement(statement: ast::Statement) -> Vec<ir::Instruction> {
    match statement {
        ast::Statement::Return(exp) => {
            match exp {
                ast::Expression::Constant(imm) => {
                    vec![
                        ir::Instruction::Mov { src: ir::Operand::Imm(imm),
                            dst: ir::Operand::Register }]
                }
            }
        }
        _ => vec![]
    }
}

