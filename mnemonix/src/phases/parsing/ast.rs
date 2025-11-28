
#[derive(Debug)]
pub enum Element {
    Directive(Directive),
    Label(String),
    Instruction(Box<Instruction>),
}

#[derive(Debug)]
pub enum Operand {
    Register(String),
    Constant(u16)
}

#[derive(Debug)]
pub struct Instruction {
    mnemonic: String,
    operands: [Operand]
}

#[derive(Debug)]
pub struct Directive {
    pub name: String
}