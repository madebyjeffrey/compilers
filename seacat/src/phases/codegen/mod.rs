pub mod ir;

pub struct Program {
    functions: FunctionDefinition,
}

pub struct FunctionDefinition {
    name: String,
    instructions: Vec<Instructions>,
}

pub enum Instructions {
    Mov { src: Operand, dst: Operand },
    Ret,
}

pub enum Operand {
    Imm(i32),
    Register,
}
