use phf_macros::phf_map;

use crate::functions::Func;
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
    Func(Func),
    Number(f64),
    Variable(Variable),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    OpeningBracket,
    ClosingBracket,
}

pub const SYMBOLS: phf::Map<&'static str, Symbol> = phf_map! {
    "(" => Symbol::OpeningBracket,
    ")" => Symbol::ClosingBracket
};

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
