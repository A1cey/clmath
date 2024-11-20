use core::fmt::Display;
use core::panic;
use phf_macros::phf_map;

use crate::error::{Error, ErrorType, TokenizerError};

use crate::functions::{Func, ELEMENTARY_FUNC_KEYWORDS, HIGHER_ORDER_FUNC_KEYWORDS};

#[derive(Debug, PartialEq, Clone)]
struct Variable {
    name: String,
    value: Option<f64>,
}

impl Variable {
    pub fn new(name: String, value: Option<f64>) -> Variable {
        Variable { name, value }
    }
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
    Function(Func),
    Number(f64),
    Variable(Variable),
    Symbol(Symbol),
    Empty,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Function,
    Number,
    Variable,
    Symbol,
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
    curr_token_type: TokenType,
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
            curr_token_type: TokenType::Empty,
            errors: Vec::new(),
        }
    }

    fn step(&mut self) {
        self.curr_idx += 1;

        if self.input_len == self.curr_idx.into() {
            self.is_done = true;
        }
    }

    fn consume(&mut self) -> Token {
        let token: Token;
        if self.token_start_idx >= self.curr_idx || self.curr_idx >= self.input_len {
            token = self.tokenize_empty();
        } else {
            let token_value = self
                .input
                .get(self.token_start_idx..self.curr_idx)
                .unwrap()
                .to_string();

            token = match self.curr_token_type {
                TokenType::Number => self.tokenize_number(&token_value),
                TokenType::Variable => self.tokenize_variable(&token_value),
                TokenType::Function => self.tokenize_function(&token_value),
                TokenType::Symbol => self.tokenize_symbol(&token_value),
                TokenType::Empty => self.tokenize_empty(),
            };
        }

        self.step();
        self.token_start_idx = self.curr_idx;

        token
    }

    fn tokenize_empty(&mut self) -> Token {
        self.add_error(TokenizerError::EmptyToken, None, None);
        Token::Empty
    }

    fn tokenize_number(&mut self, token_value: &str) -> Token {
        Token::Number(
            token_value
                .replace(",", ".")
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    self.add_error(
                        TokenizerError::InvalidNumber,
                        Some(token_value),
                        Some(err.to_string()),
                    );
                    f64::NAN
                }),
        )
    }

    fn tokenize_variable(&self, token_value: &str) -> Token {
        Token::Variable(Variable::new(token_value.to_string(), None))
    }

    fn tokenize_function(&mut self, token_value: &str) -> Token {
        match ELEMENTARY_FUNC_KEYWORDS.get(token_value) {
            Some(func) => Token::Function(Func::Elementary(func.clone())),
            None => match HIGHER_ORDER_FUNC_KEYWORDS.get(token_value) {
                Some(func) => Token::Function(Func::HigherOrder(func.clone())),
                None => {
                    self.add_error(TokenizerError::InvalidFunctionName, Some(token_value), None);
                    Token::Empty
                }
            },
        }
    }

    fn tokenize_symbol(&mut self, token_value: &str) -> Token {
        if let Some(symbol) = SYMBOLS.get(token_value) {
            match symbol {
                Symbol::OpeningBracket => Token::Symbol(Symbol::OpeningBracket),
                Symbol::ClosingBracket => Token::Symbol(Symbol::ClosingBracket),
            }
        } else {
            self.add_error(TokenizerError::InvalidSymbol, Some(token_value), None);
            Token::Empty
        }
    }

    fn add_error(
        &mut self,
        error_type: TokenizerError,
        token_value: Option<&str>,
        err_value: Option<String>,
    ) {
        let error = match error_type {
            TokenizerError::InvalidInput => {
                format!("Tokenizer could not tokenize input: {}", self.input)
            }
            TokenizerError::InvalidInputIndexing => {
                "Invalid indexing into the provided input.".to_string()
            }
            TokenizerError::EmptyToken => "Tried to tokenize an empty token.".to_string(),
            TokenizerError::InvalidFunctionName => {
                format!("{} is an invalid function name.", token_value.unwrap())
            }
            TokenizerError::InvalidSymbol => {
                format!("{} is an invalid symbol.", token_value.unwrap())
            }
            TokenizerError::InvalidNumber => {
                format!(
                    "{} is an invalid number, because \n{}",
                    token_value.unwrap(),
                    err_value.unwrap()
                )
            }
        };

        self.errors.push(Error::new(
            error,
            ErrorType::Tokenizer(error_type),
            Some(self.token_start_idx),
            Some(self.curr_idx),
        ))
    }

    fn get_char(&mut self) -> char {
        match self.input.chars().nth(self.curr_idx) {
            Some(c) => c,
            None => panic!("The index should not be greater or equal to the length of the input. This should never happen.")
        }
    }

    fn remove_empty_tokens(&mut self) {
        self.tokens.retain(|token| *token != Token::Empty);
    }
}

fn tokenize(input: String) -> Result<Vec<Token>, Vec<Error>> {
    let mut tokenizer = Tokenizer::from(input);

    while !tokenizer.is_done {}

    tokenizer.remove_empty_tokens();

    if !tokenizer.errors.is_empty() {
        return Err(tokenizer.errors);
    }
    Ok(tokenizer.tokens)
}
