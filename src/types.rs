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
    Symbol(SymbolTypes)
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionTypes {
    Addition,
    Derivative,
    Division,
    Factorial,
    Modulo,
    Multiplication,
    Subtraction,
}

pub const FUNCTIONS: &[(FunctionTypes, &'static str)] = &[
    (FunctionTypes::Addition, "+"),
    (FunctionTypes::Derivative, "der"),
    (FunctionTypes::Division, "/"),
    (FunctionTypes::Factorial, "!"),
    (FunctionTypes::Modulo, "%"),
    (FunctionTypes::Multiplication, "*"),
    (FunctionTypes::Subtraction, "-"),
];

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolTypes {
    OpeningBracket,
    ClosingBracket
}

pub const SYMBOLS: &[(SymbolTypes, &'static str)] = &[
    (SymbolTypes::OpeningBracket, "("),
    (SymbolTypes::ClosingBracket, ")"),
];

#[derive(Debug, PartialEq)]
pub enum ErrorTypes {
    IoError(String),
    ParserError(String),
}
