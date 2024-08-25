use core::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Option<isize>,
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Some(value) => write!(f, "{} = {}", self.name, value),
            None => write!(f, "{} = undefined", self.name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Func(FuncType),
    Number(f64),
    Variable(Variable),
    Symbol(SymbolType),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FuncType {
    Elementary(ElementaryFunc),
    HigherOrder(HigherOrderFunc),
}

pub const ELEMENTARY_FUNC_KEYWORDS: &[(ElementaryFunc, &'static str)] = &[
    (ElementaryFunc::Addition, "+"),
    (ElementaryFunc::Division, "/"),
    (ElementaryFunc::Factorial, "!"),
    (ElementaryFunc::Modulo, "%"),
    (ElementaryFunc::Multiplication, "*"),
    (ElementaryFunc::Subtraction, "-"),
];

#[derive(Debug, PartialEq, Clone)]
pub enum ElementaryFunc {
    Addition,
    Division,
    Factorial,
    Modulo,
    Multiplication,
    Subtraction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HigherOrderFunc {
    Derivative,
}

pub const HIGHER_ORDER_FUNC_KEYWORDS: &[(HigherOrderFunc, &'static str)] =
    &[(HigherOrderFunc::Derivative, "der")];

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolType {
    OpeningBracket,
    ClosingBracket,
}

pub const SYMBOLS: &[(SymbolType, &'static str)] = &[
    (SymbolType::OpeningBracket, "("),
    (SymbolType::ClosingBracket, ")"),
];

#[derive(Debug, PartialEq)]
pub enum ErrorType {
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
