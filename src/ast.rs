#[derive(Debug)]
pub struct Program {
    pub function: Function,
}
#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub body: Statement,
}
#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}
#[derive(Debug)]
pub enum Expression {
    Constant(i64),
}
#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}