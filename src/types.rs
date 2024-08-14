#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Option<isize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Function(FunctionTypes),
    Number(f64),
    Variable(Variable),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionTypes {
    Addition,
    Derivative,
    Division,
    Modulo,
    Multiplication,
    Subtraction,
}

pub const FUNCTIONS: &[(FunctionTypes, &str)] = &[
    (FunctionTypes::Addition, "+"),
    (FunctionTypes::Derivative, "der"),
    (FunctionTypes::Division, "/"),
    (FunctionTypes::Modulo, "%"),
    (FunctionTypes::Multiplication, "*"),
    (FunctionTypes::Subtraction, "-"),
];

#[derive(Debug, PartialEq)]
pub enum ErrorTypes {
    IoError(String),
    ParserError(String),
}
