use core::fmt::Display;
use phf_macros::phf_map;
use std::fmt::format;
use std::vec;

use crate::error::{Error, ErrorType, TokenizerError};

use crate::functions::{
    ElementaryFunc, Func, ELEMENTARY_FUNC_KEYWORDS, HIGHER_ORDER_FUNC_KEYWORDS,
};
use crate::tokenizer;

#[derive(Debug, PartialEq, Clone)]
struct Variable {
    name: String,
    value: Option<isize>,
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
enum Token {
    Func(Func),
    Number(f64),
    Variable(Variable),
    Symbol(Symbol),
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
enum Symbol {
    OpeningBracket,
    ClosingBracket,
}

const SYMBOLS: phf::Map<&'static str, Symbol> = phf_map! {
    "(" => Symbol::OpeningBracket,
    ")" => Symbol::ClosingBracket
};

enum CharOrStr<'a> {
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

struct Tokenizer {
    curr_idx: usize,
    token_start_idx: usize,
    input: String,
    input_len: usize,
    tokens: Vec<Token>,
    curr_token_type: Token,
    is_done: bool,
    errors: Vec<Error>,
}

impl Tokenizer {
    fn from(input: String) -> Self {
        Tokenizer {
            curr_idx: 0,
            token_start_idx: 0,
            input_len: input.len(),
            input,
            tokens: Vec::new(),
            is_done: false,
            curr_token_type: Token::Empty,
        }
    }

    fn step(&mut self) {
        self.curr_idx += 1;

        if self.input_len == self.curr_idx.into() {
            self.is_done = true;
        }
    }

    fn consume(&mut self) -> Token {}

    fn addError(&mut self, error_type: TokenizerError) {
        let error = match error_type {
            TokenizerError::InvalidInput => {
                format!("Tokenizer could not tokenize input: {}", self.input)
            }
            TokenizerError::InvalidInputIndexing => {
                "Invalid indexing into the provided input.".to_string()
            }
        };

        self.errors.push(Error::new(
            error,
            ErrorType::Tokenizer(error_type),
            Some(self.curr_idx),
        ))
    }

    fn get_char(&mut self) -> char {
        match self.input.get(self.curr_idx.into()) {
            Some(c) => *c,
            None => {
                self.addError(TokenizerError::InvalidInputIndexing);
                self.is_done = true;
                ' '
            }
        }
    }
}

fn tokenize(input: String) -> Result<Vec<Token>, Error> {
    let mut tokenizer = Tokenizer::from(input);

    while !tokenizer.is_done {}

    Ok(tokenizer.tokens)
}
