
pub enum Expression {
    Constant(i64),
}

pub enum Statement {
    Return(Expression),
}

pub enum FunctionDefinition {
    Function { name: String, body: Statement }
}

pub struct Program {
    pub function: FunctionDefinition,
}