pub struct Program {
    pub function: FunctionDefinition,
}

pub struct FunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

pub enum Operand {
    Imm(i64),
    Register,
}
