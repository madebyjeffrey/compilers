
#[derive(Debug)]
pub enum Expression {
    Constant(i64),
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum FunctionDefinition {
    Function { name: String, body: Statement }
}

#[derive(Debug)]
pub struct Program {
    pub function: FunctionDefinition,
}