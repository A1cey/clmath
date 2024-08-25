use std::any::TypeId;

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
    Symbol(SymbolTypes),
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
    ClosingBracket,
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

pub enum CharOrStr<'a> {
    Char(char),
    Str(&'a str),
}

impl<'a> From<char> for CharOrStr<'a> {
    fn from(c: char) -> Self {
        CharOrStr::Char(c)
    }
}

impl<'a> From<&'a str> for CharOrStr<'a> {
    fn from(s: &'a str) -> Self {
        CharOrStr::Str(s)
    }
}
