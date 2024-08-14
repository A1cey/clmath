#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: Option<isize>,
}

#[derive(Debug)]
pub enum Tokens {
    Function(FunctionTypes),
    Number(f64),
    Variable(Variable),
}

#[derive(Clone,Debug)]
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

#[derive(Debug)]
pub enum ErrorTypes {
    IoError(std::io::Error),
    ParserError(String),
}
